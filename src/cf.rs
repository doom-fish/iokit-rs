//! CoreFoundation snapshot helpers used by the safe `IOKit` wrappers.

use crate::{error::IoKitError, ffi, Result};
use core::{ffi::c_char, ptr};
use std::{collections::BTreeMap, ffi::{CStr, CString}};

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CFValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Data(Vec<u8>),
    Array(Vec<Self>),
    Dictionary(BTreeMap<String, Self>),
    Unknown(u64),
}

impl CFValue {
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::String(_) => "String",
            Self::Integer(_) => "Integer",
            Self::Boolean(_) => "Boolean",
            Self::Data(_) => "Data",
            Self::Array(_) => "Array",
            Self::Dictionary(_) => "Dictionary",
            Self::Unknown(_) => "Unknown",
        }
    }
}

#[derive(Debug)]
pub(crate) struct CFStringOwned {
    raw: ffi::CFStringRef,
}

impl CFStringOwned {
    pub(crate) fn new(value: &str) -> Result<Self> {
        let c_value = CString::new(value)
            .map_err(|_| IoKitError::InvalidArgument("string contains interior NUL byte".to_owned()))?;
        let raw = unsafe {
            ffi::CFStringCreateWithCString(
                ffi::kCFAllocatorDefault,
                c_value.as_ptr(),
                ffi::kCFStringEncodingUTF8,
            )
        };
        if raw.is_null() {
            return Err(IoKitError::UnexpectedNull("CFStringCreateWithCString"));
        }
        Ok(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> ffi::CFStringRef {
        self.raw
    }
}

impl Drop for CFStringOwned {
    fn drop(&mut self) {
        unsafe {
            ffi::CFRelease(self.raw);
        }
    }
}

pub(crate) unsafe fn string_from_cf(raw: ffi::CFStringRef) -> Option<String> {
    if raw.is_null() {
        return None;
    }

    let len = ffi::CFStringGetLength(raw);
    let cap = len.saturating_mul(4).saturating_add(1);
    let mut buffer = vec![0 as c_char; usize::try_from(cap).ok()?];
    let ok = ffi::CFStringGetCString(
        raw,
        buffer.as_mut_ptr(),
        cap,
        ffi::kCFStringEncodingUTF8,
    );
    if !ok {
        return None;
    }
    Some(CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned())
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
        let mut value = 0_i64;
        let ok = ffi::CFNumberGetValue(
            raw.cast(),
            ffi::kCFNumberSInt64Type,
            core::ptr::addr_of_mut!(value).cast(),
        );
        return if ok {
            CFValue::Integer(value)
        } else {
            CFValue::Unknown(u64::try_from(type_id).unwrap_or_default())
        };
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

    CFValue::Unknown(u64::try_from(type_id).unwrap_or_default())
}

unsafe fn key_to_string(raw: ffi::CFTypeRef) -> String {
    match value_from_ref(raw) {
        CFValue::String(value) => value,
        CFValue::Integer(value) => value.to_string(),
        CFValue::Boolean(value) => value.to_string(),
        other => format!("<{}>", other.kind_name()),
    }
}
