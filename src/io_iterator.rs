#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{bridge, io_registry::RegistryEntry, io_service::Service, object::nonnull};
use core::ffi::c_void;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct ObjectIterator {
    raw: NonNull<c_void>,
}

impl ObjectIterator {
    pub(crate) fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn is_valid(&self) -> bool {
        unsafe { bridge::iokit_swift_iterator_is_valid(self.as_ptr()) }
    }

    pub fn reset(&mut self) {
        unsafe { bridge::iokit_swift_iterator_reset(self.as_ptr()) };
    }

    pub fn next_service(&mut self) -> Option<Service> {
        Service::from_raw(unsafe { bridge::iokit_swift_iterator_next_service(self.as_ptr()) })
    }

    pub fn next_registry_entry(&mut self) -> Option<RegistryEntry> {
        RegistryEntry::from_raw(unsafe {
            bridge::iokit_swift_iterator_next_registry_entry(self.as_ptr())
        })
    }

    pub fn collect_services(mut self) -> Vec<Service> {
        let mut services = Vec::new();
        while let Some(service) = self.next_service() {
            services.push(service);
        }
        services
    }

    pub fn collect_registry_entries(mut self) -> Vec<RegistryEntry> {
        let mut entries = Vec::new();
        while let Some(entry) = self.next_registry_entry() {
            entries.push(entry);
        }
        entries
    }
}

impl Clone for ObjectIterator {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_iterator_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_iterator_retain").expect("iterator retain"),
        }
    }
}

impl Drop for ObjectIterator {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_iterator_release(self.as_ptr()) };
    }
}
