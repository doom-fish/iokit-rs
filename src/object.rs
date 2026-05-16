//! Internal helpers for bridge-owned handles and fallible conversions.

use crate::{bridge, error::IoKitError, Result};
use core::ffi::{c_char, c_void};
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

pub const fn io_result(status: i32, operation: &'static str) -> Result<()> {
    if status == 0 {
        Ok(())
    } else {
        Err(IoKitError::IoReturn(operation, status))
    }
}

pub fn c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        IoKitError::InvalidArgument(format!("string contains interior NUL byte: {value:?}"))
    })
}

pub fn nonnull(ptr: *mut c_void, what: &'static str) -> Result<NonNull<c_void>> {
    NonNull::new(ptr).ok_or(IoKitError::UnexpectedNull(what))
}

pub unsafe fn take_c_string(ptr: *mut c_char) -> Option<String> {
    let ptr = NonNull::new(ptr)?;
    let value = unsafe { CStr::from_ptr(ptr.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    unsafe { bridge::iokit_swift_free_string(ptr.as_ptr()) };
    Some(value)
}

pub unsafe fn take_required_c_string(ptr: *mut c_char, what: &'static str) -> Result<String> {
    unsafe { take_c_string(ptr) }.ok_or(IoKitError::UnexpectedNull(what))
}
