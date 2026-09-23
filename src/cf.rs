//! CoreFoundation snapshot helpers used by the safe `IOKit` wrappers.

use crate::{error::IoKitError, ffi_impl as ffi, Result};
use apple_cf::raw as cf_raw;
use core::{
    ffi::{c_char, c_void},
    ptr::{self, NonNull},
};
use std::{collections::BTreeMap, ffi::CStr};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
/// Snapshot of a Core Foundation value returned by `IOKit` APIs such as
/// `IORegistryEntryCreateCFProperties` and `IOPMCopyAssertionsByProcess`.
pub enum CFValue {
    /// Wraps a `CFStringRef` value.
    String(String),
    /// Wraps a `CFNumberRef` converted to `i64`.
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    /// Wraps a `CFBooleanRef` value.
    Boolean(bool),
    /// Wraps a `CFDataRef` value.
    Data(Vec<u8>),
    /// Wraps a `CFArrayRef` value.
    Array(Vec<Self>),
    /// Wraps a `CFDictionaryRef` value keyed by stringified entries.
    Dictionary(BTreeMap<String, Self>),
    /// Wraps an unsupported Core Foundation type by `CFTypeID`.
    Unknown(u64),
}

impl CFValue {
    /// Returns the Core Foundation-like kind name for this snapshot value.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::String(_) => "String",
            Self::Integer(_) => "Integer",
            Self::UnsignedInteger(_) => "UnsignedInteger",
            Self::Float(_) => "Float",
            Self::Boolean(_) => "Boolean",
            Self::Data(_) => "Data",
            Self::Array(_) => "Array",
            Self::Dictionary(_) => "Dictionary",
            Self::Unknown(_) => "Unknown",
        }
    }

    pub(crate) fn to_cf(&self) -> Result<OwnedCf> {
        match self {
            Self::String(value) => OwnedCf::string(value),
            Self::Integer(value) => OwnedCf::number(cf_raw::kCFNumberSInt64Type, value),
            Self::UnsignedInteger(value) => OwnedCf::unsigned(*value),
            Self::Float(value) => OwnedCf::number(cf_raw::kCFNumberFloat64Type, value),
            Self::Boolean(value) => {
                let raw = unsafe {
                    if *value {
                        cf_raw::kCFBooleanTrue
                    } else {
                        cf_raw::kCFBooleanFalse
                    }
                };
                OwnedCf::retain(raw.cast(), "kCFBoolean")
            }
            Self::Data(bytes) => {
                let length = cf_index(bytes.len())?;
                OwnedCf::adopt(
                    unsafe {
                        cf_raw::CFDataCreate(ffi::kCFAllocatorDefault, bytes.as_ptr(), length)
                    }
                    .cast(),
                    "CFDataCreate",
                )
            }
            Self::Array(items) => {
                let items = items.iter().map(Self::to_cf).collect::<Result<Vec<_>>>()?;
                let mut values = items.iter().map(OwnedCf::as_ptr).collect::<Vec<_>>();
                let count = cf_index(values.len())?;
                OwnedCf::adopt(
                    unsafe {
                        cf_raw::CFArrayCreate(
                            ffi::kCFAllocatorDefault,
                            values.as_mut_ptr(),
                            count,
                            &raw const cf_raw::kCFTypeArrayCallBacks,
                        )
                    }
                    .cast(),
                    "CFArrayCreate",
                )
            }
            Self::Dictionary(entries) => dictionary_to_cf(entries),
            Self::Unknown(type_id) => Err(IoKitError::InvalidArgument(format!(
                "a CFValue::Unknown({type_id}) snapshot cannot be converted back to Core Foundation"
            ))),
        }
    }
}

impl From<&str> for CFValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for CFValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for CFValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for CFValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u32> for CFValue {
    fn from(value: u32) -> Self {
        Self::Integer(i64::from(value))
    }
}

impl From<u64> for CFValue {
    fn from(value: u64) -> Self {
        i64::try_from(value).map_or(Self::UnsignedInteger(value), Self::Integer)
    }
}

impl From<f64> for CFValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<Vec<u8>> for CFValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Data(value)
    }
}

pub(crate) fn dictionary_to_cf(entries: &BTreeMap<String, CFValue>) -> Result<OwnedCf> {
    let keys = entries
        .keys()
        .map(|key| OwnedCf::string(key))
        .collect::<Result<Vec<_>>>()?;
    let values = entries
        .values()
        .map(CFValue::to_cf)
        .collect::<Result<Vec<_>>>()?;
    let mut key_ptrs = keys.iter().map(OwnedCf::as_ptr).collect::<Vec<_>>();
    let mut value_ptrs = values.iter().map(OwnedCf::as_ptr).collect::<Vec<_>>();
    let count = cf_index(key_ptrs.len())?;
    OwnedCf::adopt(
        unsafe {
            cf_raw::CFDictionaryCreate(
                ffi::kCFAllocatorDefault,
                key_ptrs.as_mut_ptr(),
                value_ptrs.as_mut_ptr(),
                count,
                &raw const cf_raw::kCFTypeDictionaryKeyCallBacks,
                &raw const cf_raw::kCFTypeDictionaryValueCallBacks,
            )
        }
        .cast(),
        "CFDictionaryCreate",
    )
}

const CF_NUMBER_SINT128_TYPE: cf_raw::CFNumberType = 17;

#[repr(C)]
#[derive(Default)]
struct CFSInt128 {
    high: i64,
    low: u64,
}

fn cf_index(len: usize) -> Result<ffi::CFIndex> {
    ffi::CFIndex::try_from(len).map_err(|_| {
        IoKitError::InvalidArgument(format!("{len} elements exceed the CFIndex range"))
    })
}

pub(crate) struct OwnedCf(NonNull<c_void>);

impl OwnedCf {
    fn adopt(raw: ffi::CFTypeRef, what: &'static str) -> Result<Self> {
        NonNull::new(raw.cast_mut())
            .map(Self)
            .ok_or(IoKitError::UnexpectedNull(what))
    }

    fn retain(raw: ffi::CFTypeRef, what: &'static str) -> Result<Self> {
        if raw.is_null() {
            return Err(IoKitError::UnexpectedNull(what));
        }
        Self::adopt(unsafe { ffi::CFRetain(raw) }, what)
    }

    pub(crate) fn string(value: &str) -> Result<Self> {
        let length = cf_index(value.len())?;
        Self::adopt(
            unsafe {
                cf_raw::CFStringCreateWithBytes(
                    ffi::kCFAllocatorDefault,
                    value.as_ptr(),
                    length,
                    cf_raw::kCFStringEncodingUTF8,
                    0,
                )
            }
            .cast(),
            "CFStringCreateWithBytes",
        )
    }

    fn number<T>(number_type: u32, value: &T) -> Result<Self> {
        Self::adopt(
            unsafe {
                cf_raw::CFNumberCreate(
                    ffi::kCFAllocatorDefault,
                    cf_raw::CFNumberType::from(number_type),
                    ptr::from_ref(value).cast(),
                )
            }
            .cast(),
            "CFNumberCreate",
        )
    }

    fn unsigned(value: u64) -> Result<Self> {
        if let Ok(signed) = i64::try_from(value) {
            return Self::number(cf_raw::kCFNumberSInt64Type, &signed);
        }
        let wide = CFSInt128 {
            high: 0,
            low: value,
        };
        let raw = unsafe {
            cf_raw::CFNumberCreate(
                ffi::kCFAllocatorDefault,
                CF_NUMBER_SINT128_TYPE,
                (&raw const wide).cast(),
            )
        };
        if raw.is_null() {
            Self::number(
                cf_raw::kCFNumberSInt64Type,
                &i64::from_ne_bytes(value.to_ne_bytes()),
            )
        } else {
            Self::adopt(raw.cast(), "CFNumberCreate")
        }
    }

    pub(crate) const fn as_ptr(&self) -> ffi::CFTypeRef {
        self.0.as_ptr().cast_const()
    }

    pub(crate) const fn into_raw(self) -> ffi::CFTypeRef {
        let raw = self.as_ptr();
        core::mem::forget(self);
        raw
    }
}

impl Drop for OwnedCf {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.as_ptr()) };
    }
}

pub(crate) unsafe fn string_from_cf(raw: ffi::CFStringRef) -> Option<String> {
    if raw.is_null() {
        return None;
    }

    let len = ffi::CFStringGetLength(raw);
    let cap = len.saturating_mul(4).saturating_add(1);
    let mut buffer = vec![0 as c_char; usize::try_from(cap).ok()?];
    let ok = ffi::CFStringGetCString(raw, buffer.as_mut_ptr(), cap, ffi::kCFStringEncodingUTF8);
    if !ok {
        return None;
    }
    Some(
        CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .into_owned(),
    )
}

pub(crate) unsafe fn take_value(raw: ffi::CFTypeRef) -> Option<CFValue> {
    if raw.is_null() {
        return None;
    }
    let value = value_from_ref(raw);
    ffi::CFRelease(raw);
    Some(value)
}

unsafe fn value_from_ref(raw: ffi::CFTypeRef) -> CFValue {
    let type_id = ffi::CFGetTypeID(raw);

    if type_id == ffi::CFStringGetTypeID() {
        return CFValue::String(string_from_cf(raw.cast()).unwrap_or_default());
    }

    if type_id == ffi::CFNumberGetTypeID() {
        return number_from_ref(raw.cast()).unwrap_or(CFValue::Unknown(type_id));
    }

    if type_id == ffi::CFBooleanGetTypeID() {
        return CFValue::Boolean(ffi::CFBooleanGetValue(raw.cast()));
    }

    if type_id == ffi::CFDataGetTypeID() {
        let len = ffi::CFDataGetLength(raw.cast());
        let len = usize::try_from(len).unwrap_or_default();
        let bytes = if len == 0 {
            Vec::new()
        } else {
            let ptr = ffi::CFDataGetBytePtr(raw.cast());
            std::slice::from_raw_parts(ptr, len).to_vec()
        };
        return CFValue::Data(bytes);
    }

    if type_id == ffi::CFArrayGetTypeID() {
        let count = ffi::CFArrayGetCount(raw.cast());
        let mut items = Vec::with_capacity(usize::try_from(count).unwrap_or_default());
        for index in 0..count {
            let item = ffi::CFArrayGetValueAtIndex(raw.cast(), index);
            items.push(value_from_ref(item.cast()));
        }
        return CFValue::Array(items);
    }

    if type_id == ffi::CFDictionaryGetTypeID() {
        let count = usize::try_from(ffi::CFDictionaryGetCount(raw.cast())).unwrap_or_default();
        let mut keys = vec![ptr::null(); count];
        let mut values = vec![ptr::null(); count];
        ffi::CFDictionaryGetKeysAndValues(raw.cast(), keys.as_mut_ptr(), values.as_mut_ptr());
        let mut map = BTreeMap::new();
        for (key, value) in keys.into_iter().zip(values) {
            map.insert(key_to_string(key.cast()), value_from_ref(value.cast()));
        }
        return CFValue::Dictionary(map);
    }

    CFValue::Unknown(type_id)
}

unsafe fn number_from_ref(number: ffi::CFNumberRef) -> Option<CFValue> {
    if cf_raw::CFNumberIsFloatType(number.cast()) != 0 {
        let mut value = 0_f64;
        let ok = cf_raw::CFNumberGetValue(
            number.cast(),
            cf_raw::CFNumberType::from(cf_raw::kCFNumberFloat64Type),
            (&raw mut value).cast(),
        );
        return (ok != 0).then_some(CFValue::Float(value));
    }

    let mut wide = CFSInt128::default();
    let ok = cf_raw::CFNumberGetValue(
        number.cast(),
        CF_NUMBER_SINT128_TYPE,
        (&raw mut wide).cast(),
    );
    if ok != 0 && wide.high == 0 && i64::try_from(wide.low).is_err() {
        return Some(CFValue::UnsignedInteger(wide.low));
    }

    let mut value = 0_i64;
    let ok = ffi::CFNumberGetValue(number, ffi::kCFNumberSInt64Type, (&raw mut value).cast());
    ok.then_some(CFValue::Integer(value))
}

unsafe fn key_to_string(raw: ffi::CFTypeRef) -> String {
    match value_from_ref(raw) {
        CFValue::String(value) => value,
        CFValue::Integer(value) => value.to_string(),
        CFValue::UnsignedInteger(value) => value.to_string(),
        CFValue::Float(value) => value.to_string(),
        CFValue::Boolean(value) => value.to_string(),
        other => format!("<{}>", other.kind_name()),
    }
}

#[cfg(test)]
mod tests {
    use super::{take_value, CFValue};
    use std::collections::BTreeMap;

    fn round_trip(value: &CFValue) -> CFValue {
        let owned = value.to_cf().expect("conversion to Core Foundation");
        unsafe { take_value(owned.into_raw()) }.expect("non-null value")
    }

    #[test]
    fn floats_keep_their_fraction() {
        assert_eq!(round_trip(&CFValue::Float(1.5)), CFValue::Float(1.5));
        assert_eq!(round_trip(&CFValue::Float(-0.25)), CFValue::Float(-0.25));
        assert_eq!(round_trip(&CFValue::Float(2.0)), CFValue::Float(2.0));
    }

    #[test]
    fn integers_keep_their_sign_and_width() {
        for value in [0, -1, i64::from(i32::MIN) - 1, i64::MAX, i64::MIN] {
            assert_eq!(
                round_trip(&CFValue::Integer(value)),
                CFValue::Integer(value)
            );
        }
    }

    #[test]
    fn unsigned_values_above_i64_max_stay_unsigned() {
        for value in [u64::MAX, 1 << 63, (1 << 63) + 12_345] {
            assert_eq!(
                round_trip(&CFValue::UnsignedInteger(value)),
                CFValue::UnsignedInteger(value)
            );
        }
        assert_eq!(
            round_trip(&CFValue::UnsignedInteger(7)),
            CFValue::Integer(7)
        );
    }

    #[test]
    fn from_u64_picks_the_narrowest_exact_variant() {
        assert_eq!(CFValue::from(42_u64), CFValue::Integer(42));
        assert_eq!(CFValue::from(u64::MAX), CFValue::UnsignedInteger(u64::MAX));
        assert_eq!(
            CFValue::from(u32::MAX),
            CFValue::Integer(i64::from(u32::MAX))
        );
    }

    #[test]
    fn nested_collections_round_trip() {
        let mut inner = BTreeMap::new();
        inner.insert("flag".to_owned(), CFValue::Boolean(true));
        inner.insert("bytes".to_owned(), CFValue::Data(vec![0, 1, 2, 255]));
        inner.insert("empty".to_owned(), CFValue::Array(Vec::new()));
        let mut outer = BTreeMap::new();
        outer.insert("name".to_owned(), CFValue::String("Snö ☃".to_owned()));
        outer.insert(
            "values".to_owned(),
            CFValue::Array(vec![
                CFValue::Integer(-3),
                CFValue::Float(0.5),
                CFValue::UnsignedInteger(u64::MAX),
                CFValue::Boolean(false),
            ]),
        );
        outer.insert("inner".to_owned(), CFValue::Dictionary(inner));
        let value = CFValue::Dictionary(outer);
        assert_eq!(round_trip(&value), value);
    }

    #[test]
    fn unknown_snapshots_cannot_be_written_back() {
        assert!(CFValue::Unknown(1).to_cf().is_err());
        assert!(CFValue::Array(vec![CFValue::Unknown(1)]).to_cf().is_err());
    }

    #[test]
    fn kind_names_cover_the_new_variants() {
        assert_eq!(CFValue::Float(1.0).kind_name(), "Float");
        assert_eq!(CFValue::UnsignedInteger(1).kind_name(), "UnsignedInteger");
    }
}
