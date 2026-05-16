//! Enum wrappers for `IOKit` message constants.

use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PowerMessage {
    CanSystemSleep,
    SystemWillSleep,
    SystemWillPowerOn,
    SystemHasPoweredOn,
    SystemWillNotSleep,
    Unknown(u32),
}

impl PowerMessage {
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            ffi::kIOMessageCanSystemSleep => Self::CanSystemSleep,
            ffi::kIOMessageSystemWillSleep => Self::SystemWillSleep,
            ffi::kIOMessageSystemWillPowerOn => Self::SystemWillPowerOn,
            ffi::kIOMessageSystemHasPoweredOn => Self::SystemHasPoweredOn,
            ffi::kIOMessageSystemWillNotSleep => Self::SystemWillNotSleep,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::CanSystemSleep => ffi::kIOMessageCanSystemSleep,
            Self::SystemWillSleep => ffi::kIOMessageSystemWillSleep,
            Self::SystemWillPowerOn => ffi::kIOMessageSystemWillPowerOn,
            Self::SystemHasPoweredOn => ffi::kIOMessageSystemHasPoweredOn,
            Self::SystemWillNotSleep => ffi::kIOMessageSystemWillNotSleep,
            Self::Unknown(other) => other,
        }
    }
}
