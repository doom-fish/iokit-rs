//! Internal helpers for `IOKit` object ownership.

use crate::{error::IoKitError, ffi};

#[derive(Debug)]
pub struct IoObject {
    raw: ffi::io_object_t,
}

impl IoObject {
    pub const fn new(raw: ffi::io_object_t) -> Self {
        Self { raw }
    }

    pub const fn as_raw(&self) -> ffi::io_object_t {
        self.raw
    }
}

impl Clone for IoObject {
    fn clone(&self) -> Self {
        let status = unsafe { ffi::IOObjectRetain(self.raw) };
        debug_assert_eq!(status, ffi::kIOReturnSuccess);
        Self { raw: self.raw }
    }
}

impl Drop for IoObject {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::IOObjectRelease(self.raw);
        }
    }
}

pub const fn io_result(status: i32, operation: &'static str) -> crate::Result<()> {
    if status == ffi::kIOReturnSuccess {
        Ok(())
    } else {
        Err(IoKitError::IoReturn(operation, status))
    }
}
