//! `IOService` registry lookup and traversal helpers.

use crate::{
    cf::{take_value, CFStringOwned, CFValue},
    error::IoKitError,
    ffi,
    object::{io_result, IoObject},
    Result,
};
use core::ffi::c_char;
use std::ffi::{CStr, CString};

pub const SERVICE_PLANE: &str = ffi::K_IO_SERVICE_PLANE;

#[derive(Debug, Clone)]
pub struct Service {
    obj: IoObject,
}

#[derive(Debug)]
pub struct ObjectIterator {
    obj: IoObject,
}

impl Service {
    pub(crate) fn from_raw(raw: ffi::io_service_t) -> Option<Self> {
        (raw != 0).then(|| Self { obj: IoObject::new(raw) })
    }

    const fn as_raw(&self) -> ffi::io_service_t {
        self.obj.as_raw()
    }

    /// Return the kernel class name of this registry object.
    ///
    /// # Errors
    ///
    /// Returns the underlying `IOReturn` if `IOObjectGetClass` fails.
    pub fn class_name(&self) -> Result<String> {
        let mut buffer = [0 as c_char; ffi::IO_NAME_SIZE];
        let status = unsafe { ffi::IOObjectGetClass(self.as_raw(), buffer.as_mut_ptr()) };
        io_result(status, "IOObjectGetClass")?;
        Ok(unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_string_lossy()
            .into_owned())
    }

    /// Copy a CoreFoundation snapshot of a registry property.
    ///
    /// # Errors
    ///
    /// Returns an error if the property key contains interior NUL bytes.
    pub fn property(&self, key: &str) -> Result<Option<CFValue>> {
        let key = CFStringOwned::new(key)?;
        Ok(unsafe {
            take_value(ffi::IORegistryEntryCreateCFProperty(
                self.as_raw(),
                key.as_raw(),
                ffi::kCFAllocatorDefault,
                0,
            ))
        })
    }

    /// Return the first parent entry in the specified plane.
    ///
    /// # Errors
    ///
    /// Returns an error if the plane name contains interior NUL bytes.
    pub fn parent(&self, plane: &str) -> Result<Option<Self>> {
        let plane = c_string(plane)?;
        let mut parent = 0;
        let status = unsafe {
            ffi::IORegistryEntryGetParentEntry(self.as_raw(), plane.as_ptr(), &mut parent)
        };
        if parent == 0 {
            return Ok(None);
        }
        io_result(status, "IORegistryEntryGetParentEntry")?;
        Ok(Self::from_raw(parent))
    }

    /// Return child entries in the specified plane.
    ///
    /// # Errors
    ///
    /// Returns an error if the plane name contains interior NUL bytes or the
    /// underlying `IORegistryEntryGetChildIterator` call fails.
    pub fn children(&self, plane: &str) -> Result<Vec<Self>> {
        let plane = c_string(plane)?;
        let mut iterator = 0;
        let status = unsafe {
            ffi::IORegistryEntryGetChildIterator(self.as_raw(), plane.as_ptr(), &mut iterator)
        };
        io_result(status, "IORegistryEntryGetChildIterator")?;
        Ok(ObjectIterator::from_raw(iterator)
            .map_or_else(Vec::new, ObjectIterator::collect_services))
    }
}

impl ObjectIterator {
    pub(crate) fn from_raw(raw: ffi::io_iterator_t) -> Option<Self> {
        (raw != 0).then(|| Self { obj: IoObject::new(raw) })
    }

    const fn as_raw(&self) -> ffi::io_iterator_t {
        self.obj.as_raw()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IOIteratorIsValid(self.as_raw()) != 0 }
    }

    pub fn reset(&mut self) {
        unsafe { ffi::IOIteratorReset(self.as_raw()) };
    }

    #[must_use]
    pub fn next_service(&mut self) -> Option<Service> {
        Service::from_raw(unsafe { ffi::IOIteratorNext(self.as_raw()) })
    }

    #[must_use]
    pub fn collect_services(mut self) -> Vec<Service> {
        let mut out = Vec::new();
        while let Some(service) = self.next_service() {
            out.push(service);
        }
        out
    }
}

/// Return the first matching `IOService` instance for the requested class.
///
/// # Errors
///
/// Returns an error if the class name contains interior NUL bytes or if
/// `IOServiceMatching` unexpectedly returns `NULL`.
pub fn matching_service(class_name: &str) -> Result<Option<Service>> {
    let class_name = c_string(class_name)?;
    let matching = unsafe { ffi::IOServiceMatching(class_name.as_ptr()) };
    if matching.is_null() {
        return Err(IoKitError::UnexpectedNull("IOServiceMatching"));
    }
    Ok(Service::from_raw(unsafe {
        ffi::IOServiceGetMatchingService(0, matching.cast())
    }))
}

/// Return all currently matched `IOService` instances for the requested class.
///
/// # Errors
///
/// Returns an error if the class name contains interior NUL bytes, if
/// `IOServiceMatching` unexpectedly returns `NULL`, or if
/// `IOServiceGetMatchingServices` fails.
pub fn matching_services(class_name: &str) -> Result<Vec<Service>> {
    let class_name = c_string(class_name)?;
    let matching = unsafe { ffi::IOServiceMatching(class_name.as_ptr()) };
    if matching.is_null() {
        return Err(IoKitError::UnexpectedNull("IOServiceMatching"));
    }

    let mut iterator = 0;
    let status = unsafe { ffi::IOServiceGetMatchingServices(0, matching.cast(), &mut iterator) };
    io_result(status, "IOServiceGetMatchingServices")?;
    Ok(ObjectIterator::from_raw(iterator).map_or_else(Vec::new, ObjectIterator::collect_services))
}

fn c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        IoKitError::InvalidArgument(format!("string contains interior NUL byte: {value:?}"))
    })
}
