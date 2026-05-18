//! Safe wrappers around `IOService*` matching and lookup APIs.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    bridge,
    error::Result,
    ffi_impl,
    io_connect::Connect,
    io_iterator::ObjectIterator,
    io_registry::RegistryEntry,
    object::{c_string, io_result, nonnull, take_required_c_string},
};
use core::ffi::c_void;
use std::ptr::NonNull;

/// Wraps `K_IO_SERVICE_PLANE`.
pub const SERVICE_PLANE: &str = ffi_impl::K_IO_SERVICE_PLANE;
/// Wraps `K_IOPublishNotification`.
pub const PUBLISH_NOTIFICATION: &str = ffi_impl::K_IOPublishNotification;
/// Wraps `K_IOFirstPublishNotification`.
pub const FIRST_PUBLISH_NOTIFICATION: &str = ffi_impl::K_IOFirstPublishNotification;
/// Wraps `K_IOMatchedNotification`.
pub const MATCHED_NOTIFICATION: &str = ffi_impl::K_IOMatchedNotification;
/// Wraps `K_IOFirstMatchNotification`.
pub const FIRST_MATCH_NOTIFICATION: &str = ffi_impl::K_IOFirstMatchNotification;
/// Wraps `K_IOTerminatedNotification`.
pub const TERMINATED_NOTIFICATION: &str = ffi_impl::K_IOTerminatedNotification;
/// Wraps `K_IOGeneralInterest`.
pub const GENERAL_INTEREST: &str = ffi_impl::K_IOGeneralInterest;
/// Wraps `K_IOBusyInterest`.
pub const BUSY_INTEREST: &str = ffi_impl::K_IOBusyInterest;
/// Wraps `kIOServiceInteractionAllowed`.
pub const SERVICE_INTERACTION_ALLOWED: u32 = ffi_impl::kIOServiceInteractionAllowed;

#[derive(Debug)]
/// Safe retained wrapper around an `io_service_t` handle.
pub struct Service {
    raw: NonNull<c_void>,
}

impl Service {
    pub(crate) fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    /// Returns the service class name.
    pub fn class_name(&self) -> Result<String> {
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_service_class_name(self.as_ptr()),
                "iokit_swift_service_class_name",
            )
        }
    }

    /// Returns the bundle identifier for the service class, if available.
    pub fn bundle_identifier(&self) -> Option<String> {
        unsafe {
            crate::object::take_c_string(bridge::iokit_swift_service_bundle_identifier(
                self.as_ptr(),
            ))
        }
    }

    /// Returns the superclass name for the service class, if available.
    pub fn superclass_name(&self) -> Option<String> {
        unsafe {
            crate::object::take_c_string(bridge::iokit_swift_service_superclass_name(self.as_ptr()))
        }
    }

    /// Wraps `IOObjectConformsTo` for this service.
    pub fn conforms_to(&self, class_name: &str) -> Result<bool> {
        let class_name = c_string(class_name)?;
        Ok(unsafe { bridge::iokit_swift_service_conforms_to(self.as_ptr(), class_name.as_ptr()) })
    }

    /// Wraps `IOObjectIsEqualTo`.
    pub fn is_equal_to(&self, other: &Self) -> bool {
        unsafe { bridge::iokit_swift_service_is_equal_to(self.as_ptr(), other.as_ptr()) }
    }

    /// Returns the kernel retain count for this service object.
    pub fn kernel_retain_count(&self) -> u32 {
        unsafe { bridge::iokit_swift_service_kernel_retain_count(self.as_ptr()) }
    }

    /// Returns the user-space retain count for this service object.
    pub fn user_retain_count(&self) -> u32 {
        unsafe { bridge::iokit_swift_service_user_retain_count(self.as_ptr()) }
    }

    /// Returns the combined retain count for this service object.
    pub fn retain_count(&self) -> u32 {
        unsafe { bridge::iokit_swift_service_retain_count(self.as_ptr()) }
    }

    /// Wraps `IOServiceGetBusyState`.
    pub fn busy_state(&self) -> Result<u32> {
        let mut busy_state = 0_u32;
        io_result(
            unsafe { bridge::iokit_swift_service_busy_state(self.as_ptr(), &mut busy_state) },
            "IOServiceGetBusyState",
        )?;
        Ok(busy_state)
    }

    /// Wraps `IOServiceWaitQuiet`.
    pub fn wait_quiet(&self, seconds: u32) -> Result<()> {
        io_result(
            unsafe { bridge::iokit_swift_service_wait_quiet(self.as_ptr(), seconds) },
            "IOServiceWaitQuiet",
        )
    }

    /// Wraps `IOServiceAuthorize`.
    pub fn authorize(&self, options: u32) -> Result<()> {
        io_result(
            unsafe { bridge::iokit_swift_service_authorize(self.as_ptr(), options) },
            "IOServiceAuthorize",
        )
    }

    /// Wraps `IOServiceOpen`.
    pub fn open(&self, ty: u32) -> Result<Connect> {
        let mut raw = core::ptr::null_mut();
        io_result(
            unsafe { bridge::iokit_swift_service_open(self.as_ptr(), ty, &mut raw) },
            "IOServiceOpen",
        )?;
        Connect::from_raw(raw).ok_or(crate::IoKitError::UnexpectedNull(
            "iokit_swift_service_open",
        ))
    }

    /// Copies a single registry property for this service.
    pub fn property(&self, key: &str) -> Result<Option<crate::CFValue>> {
        let key = c_string(key)?;
        Ok(unsafe {
            crate::cf::take_value(
                bridge::iokit_swift_registry_entry_property(self.as_ptr(), key.as_ptr()).cast(),
            )
        })
    }

    /// Copies the full registry property dictionary for this service.
    pub fn properties(&self) -> Option<crate::CFValue> {
        unsafe {
            crate::cf::take_value(
                bridge::iokit_swift_registry_entry_properties(self.as_ptr()).cast(),
            )
        }
    }

    /// Wraps `IORegistryEntrySearchCFProperty` for this service.
    pub fn search_property(
        &self,
        plane: &str,
        key: &str,
        options: u32,
    ) -> Result<Option<crate::CFValue>> {
        let plane = c_string(plane)?;
        let key = c_string(key)?;
        Ok(unsafe {
            crate::cf::take_value(
                bridge::iokit_swift_registry_entry_search_property(
                    self.as_ptr(),
                    plane.as_ptr(),
                    key.as_ptr(),
                    options,
                )
                .cast(),
            )
        })
    }

    /// Returns the parent service in the given plane.
    pub fn parent(&self, plane: &str) -> Result<Option<Self>> {
        let plane = c_string(plane)?;
        Ok(Self::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_parent(self.as_ptr(), plane.as_ptr())
        }))
    }

    /// Returns the first child service in the given plane.
    pub fn child(&self, plane: &str) -> Result<Option<Self>> {
        let plane = c_string(plane)?;
        Ok(Self::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_child(self.as_ptr(), plane.as_ptr())
        }))
    }

    /// Returns an iterator over parent services in the given plane.
    pub fn parent_iterator(&self, plane: &str) -> Result<Option<ObjectIterator>> {
        let plane = c_string(plane)?;
        Ok(ObjectIterator::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_parent_iterator(self.as_ptr(), plane.as_ptr())
        }))
    }

    /// Returns an iterator over child services in the given plane.
    pub fn child_iterator(&self, plane: &str) -> Result<Option<ObjectIterator>> {
        let plane = c_string(plane)?;
        Ok(ObjectIterator::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_child_iterator(self.as_ptr(), plane.as_ptr())
        }))
    }

    /// Collects all child services in the given plane.
    pub fn children(&self, plane: &str) -> Result<Vec<Self>> {
        Ok(self
            .child_iterator(plane)?
            .map_or_else(Vec::new, ObjectIterator::collect_services))
    }

    /// Wraps `IORegistryEntryGetRegistryEntryID`.
    pub fn registry_entry_id(&self) -> Result<u64> {
        let mut entry_id = 0_u64;
        io_result(
            unsafe {
                bridge::iokit_swift_registry_entry_registry_entry_id(self.as_ptr(), &mut entry_id)
            },
            "IORegistryEntryGetRegistryEntryID",
        )?;
        Ok(entry_id)
    }

    /// Wraps `IORegistryEntryGetPath` for this service.
    pub fn path(&self, plane: &str) -> Result<String> {
        let plane = c_string(plane)?;
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_path(self.as_ptr(), plane.as_ptr()),
                "iokit_swift_registry_entry_path",
            )
        }
    }

    /// Wraps `IORegistryEntryGetName` for this service.
    pub fn name(&self) -> Result<String> {
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_name(self.as_ptr()),
                "iokit_swift_registry_entry_name",
            )
        }
    }

    /// Wraps `IORegistryEntryGetNameInPlane` for this service.
    pub fn name_in_plane(&self, plane: &str) -> Result<String> {
        let plane = c_string(plane)?;
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_name_in_plane(self.as_ptr(), plane.as_ptr()),
                "iokit_swift_registry_entry_name_in_plane",
            )
        }
    }

    /// Wraps `IORegistryEntryGetLocationInPlane` for this service.
    pub fn location_in_plane(&self, plane: &str) -> Result<String> {
        let plane = c_string(plane)?;
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_location_in_plane(self.as_ptr(), plane.as_ptr()),
                "iokit_swift_registry_entry_location_in_plane",
            )
        }
    }

    /// Reports whether this service exists in the given plane.
    pub fn in_plane(&self, plane: &str) -> Result<bool> {
        let plane = c_string(plane)?;
        Ok(unsafe { bridge::iokit_swift_registry_entry_in_plane(self.as_ptr(), plane.as_ptr()) })
    }

    /// Returns this service as a `RegistryEntry`.
    pub fn registry_entry(&self) -> Result<RegistryEntry> {
        RegistryEntry::from_raw(unsafe {
            bridge::iokit_swift_service_as_registry_entry(self.as_ptr())
        })
        .ok_or(crate::IoKitError::UnexpectedNull(
            "iokit_swift_service_as_registry_entry",
        ))
    }
}

/// Clones the retained service handle.
impl Clone for Service {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_service_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_service_retain").expect("service retain"),
        }
    }
}

/// Releases the retained service handle on drop.
impl Drop for Service {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_service_release(self.as_ptr()) };
    }
}

/// Returns the first service matching a class name.
pub fn matching_service(class_name: &str) -> Result<Option<Service>> {
    let class_name = c_string(class_name)?;
    Ok(Service::from_raw(unsafe {
        bridge::iokit_swift_service_matching(class_name.as_ptr())
    }))
}

/// Returns the first service matching a service name.
pub fn name_matching_service(service_name: &str) -> Result<Option<Service>> {
    let service_name = c_string(service_name)?;
    Ok(Service::from_raw(unsafe {
        bridge::iokit_swift_service_name_matching(service_name.as_ptr())
    }))
}

/// Returns the service with the given registry entry ID, if present.
pub fn matching_service_entry_id(entry_id: u64) -> Option<Service> {
    Service::from_raw(unsafe { bridge::iokit_swift_service_matching_entry_id(entry_id) })
}

/// Returns an iterator over services matching a class name.
pub fn matching_services_iterator(class_name: &str) -> Result<Option<ObjectIterator>> {
    let class_name = c_string(class_name)?;
    Ok(ObjectIterator::from_raw(unsafe {
        bridge::iokit_swift_matching_services(class_name.as_ptr())
    }))
}

/// Returns an iterator over services matching a service name.
pub fn name_matching_services_iterator(service_name: &str) -> Result<Option<ObjectIterator>> {
    let service_name = c_string(service_name)?;
    Ok(ObjectIterator::from_raw(unsafe {
        bridge::iokit_swift_name_matching_services(service_name.as_ptr())
    }))
}

/// Collects all services matching a class name.
pub fn matching_services(class_name: &str) -> Result<Vec<Service>> {
    Ok(matching_services_iterator(class_name)?
        .map_or_else(Vec::new, ObjectIterator::collect_services))
}

/// Collects all services matching a service name.
pub fn name_matching_services(service_name: &str) -> Result<Vec<Service>> {
    Ok(name_matching_services_iterator(service_name)?
        .map_or_else(Vec::new, ObjectIterator::collect_services))
}
