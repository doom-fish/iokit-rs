#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    bridge,
    error::Result,
    object::{io_result, nonnull},
};
use core::ffi::c_void;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct NotificationPort {
    raw: NonNull<c_void>,
}

impl NotificationPort {
    pub fn new() -> Result<Self> {
        let raw = unsafe { bridge::iokit_swift_notification_port_create() };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_notification_port_create")?,
        })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn mach_port(&self) -> u32 {
        unsafe { bridge::iokit_swift_notification_port_mach_port(self.as_ptr()) }
    }

    pub fn run_loop_source_raw(&self) -> usize {
        unsafe { bridge::iokit_swift_notification_port_run_loop_source(self.as_ptr()) as usize }
    }

    pub fn set_importance_receiver(&self) -> Result<()> {
        io_result(
            unsafe { bridge::iokit_swift_notification_port_set_importance_receiver(self.as_ptr()) },
            "IONotificationPortSetImportanceReceiver",
        )
    }
}

impl Clone for NotificationPort {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_notification_port_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_notification_port_retain")
                .expect("notification port retain"),
        }
    }
}

impl Drop for NotificationPort {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_notification_port_release(self.as_ptr()) };
    }
}
