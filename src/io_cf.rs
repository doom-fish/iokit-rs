#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    cf::{string_from_cf, take_value},
    error::Result,
    ffi_impl,
    object::c_string,
    CFValue,
};

/// Wraps `kIOCFSerializeToBinary`.
pub const SERIALIZE_TO_BINARY: ffi_impl::CFOptionFlags = ffi_impl::kIOCFSerializeToBinary;

unsafe fn take_error_string(error_string: ffi_impl::CFStringRef) -> String {
    if error_string.is_null() {
        return "unknown IOCFUnserialize failure".to_string();
    }

    let message = unsafe { string_from_cf(error_string) }
        .unwrap_or_else(|| "unknown IOCFUnserialize failure".to_string());
    unsafe { ffi_impl::CFRelease(error_string.cast()) };
    message
}

/// Wraps `IOCFSerialize`.
///
/// # Safety
///
/// `object` must be a valid `CFTypeRef` accepted by `IOCFSerialize`.
pub unsafe fn serialize_raw(
    object: ffi_impl::CFTypeRef,
    options: ffi_impl::CFOptionFlags,
) -> Option<Vec<u8>> {
    match unsafe { take_value(ffi_impl::IOCFSerialize(object, options).cast()) } {
        Some(CFValue::Data(bytes)) => Some(bytes),
        _ => None,
    }
}

/// Wraps `IOCFUnserialize`.
pub fn unserialize(buffer: &str) -> Result<CFValue> {
    let buffer = c_string(buffer)?;
    let mut error_string = core::ptr::null();
    let value = unsafe {
        ffi_impl::IOCFUnserialize(
            buffer.as_ptr(),
            ffi_impl::kCFAllocatorDefault,
            0,
            &mut error_string,
        )
    };
    if value.is_null() {
        return Err(crate::IoKitError::InvalidArgument(unsafe {
            take_error_string(error_string)
        }));
    }

    unsafe { take_value(value) }.ok_or(crate::IoKitError::UnexpectedNull("IOCFUnserialize"))
}

/// Wraps `IOCFUnserializeBinary`.
pub fn unserialize_binary(buffer: &[u8]) -> Result<CFValue> {
    let mut error_string = core::ptr::null();
    let value = unsafe {
        ffi_impl::IOCFUnserializeBinary(
            buffer.as_ptr().cast(),
            buffer.len(),
            ffi_impl::kCFAllocatorDefault,
            0,
            &mut error_string,
        )
    };
    if value.is_null() {
        return Err(crate::IoKitError::InvalidArgument(unsafe {
            take_error_string(error_string)
        }));
    }

    unsafe { take_value(value) }.ok_or(crate::IoKitError::UnexpectedNull("IOCFUnserializeBinary"))
}

/// Wraps `IOCFUnserializeWithSize`.
pub fn unserialize_with_size(buffer: &[u8]) -> Result<CFValue> {
    let mut error_string = core::ptr::null();
    let value = unsafe {
        ffi_impl::IOCFUnserializeWithSize(
            buffer.as_ptr().cast(),
            buffer.len(),
            ffi_impl::kCFAllocatorDefault,
            0,
            &mut error_string,
        )
    };
    if value.is_null() {
        return Err(crate::IoKitError::InvalidArgument(unsafe {
            take_error_string(error_string)
        }));
    }

    unsafe { take_value(value) }.ok_or(crate::IoKitError::UnexpectedNull("IOCFUnserializeWithSize"))
}
