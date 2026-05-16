//! Power-management helpers built on top of `IOPMLib`.

use crate::{
    cf::{take_value, CFStringOwned, CFValue},
    ffi,
    object::io_result,
    types::PowerMessage,
    Result,
};
use std::collections::BTreeMap;

pub const ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP: &str = "PreventUserIdleSystemSleep";
pub const ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP: &str = "PreventUserIdleDisplaySleep";
pub const ASSERT_PREVENT_DISK_IDLE: &str = "PreventDiskIdle";
pub const ASSERT_NETWORK_CLIENT_ACTIVE: &str = "NetworkClientActive";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerAssertionLevel {
    Off,
    On,
}

impl PowerAssertionLevel {
    const fn as_raw(self) -> ffi::IOPMAssertionLevel {
        match self {
            Self::Off => ffi::kIOPMAssertionLevelOff,
            Self::On => ffi::kIOPMAssertionLevelOn,
        }
    }
}

#[derive(Debug)]
pub struct PowerAssertion {
    id: ffi::IOPMAssertionID,
}

impl PowerAssertion {
    /// Create a level-on power assertion with a descriptive name.
    ///
    /// # Errors
    ///
    /// Returns any `IOReturn` emitted by `IOPMAssertionCreateWithName`.
    pub fn create(assertion_type: &str, name: &str) -> Result<Self> {
        Self::create_with_level(assertion_type, PowerAssertionLevel::On, name)
    }

    /// Create a power assertion with the requested level and name.
    ///
    /// # Errors
    ///
    /// Returns any `IOReturn` emitted by `IOPMAssertionCreateWithName`.
    pub fn create_with_level(
        assertion_type: &str,
        level: PowerAssertionLevel,
        name: &str,
    ) -> Result<Self> {
        let assertion_type = CFStringOwned::new(assertion_type)?;
        let assertion_name = CFStringOwned::new(name)?;
        let mut id = 0;
        let status = unsafe {
            ffi::IOPMAssertionCreateWithName(
                assertion_type.as_raw(),
                level.as_raw(),
                assertion_name.as_raw(),
                &mut id,
            )
        };
        io_result(status, "IOPMAssertionCreateWithName")?;
        Ok(Self { id })
    }

    #[must_use]
    pub const fn id(&self) -> ffi::IOPMAssertionID {
        self.id
    }
}

impl Drop for PowerAssertion {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::IOPMAssertionRelease(self.id);
        }
    }
}

/// Copy the current system sleep assertions grouped by owning PID.
///
/// # Errors
///
/// Returns any `IOReturn` emitted by `IOPMCopyAssertionsByProcess`.
pub fn copy_assertions_by_process() -> Result<CFValue> {
    let mut assertions = core::ptr::null();
    let status = unsafe { ffi::IOPMCopyAssertionsByProcess(&mut assertions) };
    io_result(status, "IOPMCopyAssertionsByProcess")?;
    Ok(unsafe { take_value(assertions.cast()) }
        .unwrap_or_else(|| CFValue::Dictionary(BTreeMap::default())))
}

/// Convert a raw `IOMessage.h` constant into a typed sleep/wake message enum.
#[must_use]
pub const fn power_message_from_raw(raw: u32) -> PowerMessage {
    PowerMessage::from_raw(raw)
}
