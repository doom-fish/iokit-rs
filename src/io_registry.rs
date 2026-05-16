#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    bridge,
    cf::take_value,
    error::Result,
    ffi_impl,
    io_iterator::ObjectIterator,
    object::{c_string, io_result, nonnull, take_required_c_string},
    CFValue,
};
use core::ffi::c_void;
use std::ptr::NonNull;

pub const SERVICE_PLANE: &str = ffi_impl::K_IO_SERVICE_PLANE;
pub const REGISTRY_ITERATE_RECURSIVELY: u32 = ffi_impl::kIORegistryIterateRecursively;
pub const REGISTRY_ITERATE_PARENTS: u32 = ffi_impl::kIORegistryIterateParents;

#[derive(Debug)]
pub struct RegistryEntry {
    raw: NonNull<c_void>,
}

impl RegistryEntry {
    pub(crate) fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn from_path(path: &str) -> Result<Option<Self>> {
        let path = c_string(path)?;
        Ok(Self::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_from_path(path.as_ptr())
        }))
    }

    pub fn name(&self) -> Result<String> {
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_name(self.as_ptr()),
                "iokit_swift_registry_entry_name",
            )
        }
    }

    pub fn name_in_plane(&self, plane: &str) -> Result<String> {
        let plane = c_string(plane)?;
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_name_in_plane(self.as_ptr(), plane.as_ptr()),
                "iokit_swift_registry_entry_name_in_plane",
            )
        }
    }

    pub fn location_in_plane(&self, plane: &str) -> Result<String> {
        let plane = c_string(plane)?;
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_location_in_plane(self.as_ptr(), plane.as_ptr()),
                "iokit_swift_registry_entry_location_in_plane",
            )
        }
    }

    pub fn path(&self, plane: &str) -> Result<String> {
        let plane = c_string(plane)?;
        unsafe {
            take_required_c_string(
                bridge::iokit_swift_registry_entry_path(self.as_ptr(), plane.as_ptr()),
                "iokit_swift_registry_entry_path",
            )
        }
    }

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

    pub fn properties(&self) -> Option<CFValue> {
        unsafe { take_value(bridge::iokit_swift_registry_entry_properties(self.as_ptr()).cast()) }
    }

    pub fn property(&self, key: &str) -> Result<Option<CFValue>> {
        let key = c_string(key)?;
        Ok(unsafe {
            take_value(
                bridge::iokit_swift_registry_entry_property(self.as_ptr(), key.as_ptr()).cast(),
            )
        })
    }

    pub fn search_property(&self, plane: &str, key: &str, options: u32) -> Result<Option<CFValue>> {
        let plane = c_string(plane)?;
        let key = c_string(key)?;
        Ok(unsafe {
            take_value(
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

    pub fn parent(&self, plane: &str) -> Result<Option<Self>> {
        let plane = c_string(plane)?;
        Ok(Self::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_parent(self.as_ptr(), plane.as_ptr())
        }))
    }

    pub fn child(&self, plane: &str) -> Result<Option<Self>> {
        let plane = c_string(plane)?;
        Ok(Self::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_child(self.as_ptr(), plane.as_ptr())
        }))
    }

    pub fn parent_iterator(&self, plane: &str) -> Result<Option<ObjectIterator>> {
        let plane = c_string(plane)?;
        Ok(ObjectIterator::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_parent_iterator(self.as_ptr(), plane.as_ptr())
        }))
    }

    pub fn child_iterator(&self, plane: &str) -> Result<Option<ObjectIterator>> {
        let plane = c_string(plane)?;
        Ok(ObjectIterator::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_child_iterator(self.as_ptr(), plane.as_ptr())
        }))
    }

    pub fn create_iterator(&self, plane: &str, options: u32) -> Result<Option<ObjectIterator>> {
        let plane = c_string(plane)?;
        Ok(ObjectIterator::from_raw(unsafe {
            bridge::iokit_swift_registry_entry_create_iterator(
                self.as_ptr(),
                plane.as_ptr(),
                options,
            )
        }))
    }

    pub fn in_plane(&self, plane: &str) -> Result<bool> {
        let plane = c_string(plane)?;
        Ok(unsafe { bridge::iokit_swift_registry_entry_in_plane(self.as_ptr(), plane.as_ptr()) })
    }
}

impl Clone for RegistryEntry {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_registry_entry_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_registry_entry_retain").expect("registry retain"),
        }
    }
}

impl Drop for RegistryEntry {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_registry_entry_release(self.as_ptr()) };
    }
}
