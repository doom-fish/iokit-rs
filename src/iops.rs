//! Safe wrappers around `IOPS*` power-source APIs.

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
    object::{nonnull, take_c_string},
    CFValue,
};
use core::ffi::c_void;
use std::ptr::NonNull;

/// Wraps `K_IOPSNotifyLowBattery`.
pub const NOTIFY_LOW_BATTERY: &str = ffi_impl::K_IOPSNotifyLowBattery;
/// Wraps `K_IOPSNotifyTimeRemaining`.
pub const NOTIFY_TIME_REMAINING: &str = ffi_impl::K_IOPSNotifyTimeRemaining;
/// Wraps `K_IOPSNotifyPowerSource`.
pub const NOTIFY_POWER_SOURCE: &str = ffi_impl::K_IOPSNotifyPowerSource;
/// Wraps `K_IOPSNotifyAttach`.
pub const NOTIFY_ATTACH: &str = ffi_impl::K_IOPSNotifyAttach;
/// Wraps `K_IOPSNotifyAnyPowerSource`.
pub const NOTIFY_ANY_POWER_SOURCE: &str = ffi_impl::K_IOPSNotifyAnyPowerSource;
/// Wraps `K_IOPSPowerSourcesNotificationKey`.
pub const POWER_SOURCES_NOTIFICATION_KEY: &str = ffi_impl::K_IOPSPowerSourcesNotificationKey;
/// Wraps `K_IOPSTimeRemainingNotificationKey`.
pub const TIME_REMAINING_NOTIFICATION_KEY: &str = ffi_impl::K_IOPSTimeRemainingNotificationKey;
/// Wraps `K_IOPMACPowerKey`.
pub const PROVIDING_POWER_AC: &str = ffi_impl::K_IOPMACPowerKey;
/// Wraps `K_IOPMBatteryPowerKey`.
pub const PROVIDING_POWER_BATTERY: &str = ffi_impl::K_IOPMBatteryPowerKey;
/// Wraps `K_IOPMUPSPowerKey`.
pub const PROVIDING_POWER_UPS: &str = ffi_impl::K_IOPMUPSPowerKey;
/// Wraps `kIOPSTimeRemainingUnknown`.
pub const TIME_REMAINING_UNKNOWN: f64 = ffi_impl::kIOPSTimeRemainingUnknown;
/// Wraps `kIOPSTimeRemainingUnlimited`.
pub const TIME_REMAINING_UNLIMITED: f64 = ffi_impl::kIOPSTimeRemainingUnlimited;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Typed wrapper around public `IOPSLowBatteryWarning` values.
pub enum BatteryWarningLevel {
    /// Wraps `kIOPSLowBatteryWarningNone`.
    None,
    /// Wraps `kIOPSLowBatteryWarningEarly`.
    Early,
    /// Wraps `kIOPSLowBatteryWarningFinal`.
    Final,
    /// Wraps an unrecognized battery warning level.
    Unknown(u32),
}

impl BatteryWarningLevel {
    /// Builds a `BatteryWarningLevel` from a raw power-source warning code.
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            ffi_impl::kIOPSLowBatteryWarningNone => Self::None,
            ffi_impl::kIOPSLowBatteryWarningEarly => Self::Early,
            ffi_impl::kIOPSLowBatteryWarningFinal => Self::Final,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug)]
/// Safe retained wrapper around an `IOPSCopyPowerSourcesInfo` snapshot.
pub struct PowerSourcesInfo {
    raw: NonNull<c_void>,
}

impl PowerSourcesInfo {
    /// Copies the current power-sources snapshot.
    pub fn copy() -> Result<Self> {
        let raw = unsafe { bridge::iokit_swift_iops_power_sources_info_create() };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_iops_power_sources_info_create")?,
        })
    }

    const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    /// Returns the number of power sources in this snapshot.
    pub fn len(&self) -> usize {
        unsafe { bridge::iokit_swift_iops_power_sources_info_count(self.as_ptr()) }
    }

    /// Returns `true` when this snapshot contains no power sources.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies the description dictionary for a power source by index.
    pub fn description(&self, index: usize) -> Option<CFValue> {
        unsafe {
            take_value(
                bridge::iokit_swift_iops_power_sources_info_description(self.as_ptr(), index)
                    .cast(),
            )
        }
    }

    /// Returns the currently active power-source type string.
    pub fn providing_power_source_type(&self) -> Option<String> {
        unsafe {
            take_c_string(bridge::iokit_swift_iops_power_sources_info_provider_type(
                self.as_ptr(),
            ))
        }
    }
}

/// Clones the retained power-sources snapshot handle.
impl Clone for PowerSourcesInfo {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_iops_power_sources_info_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_iops_power_sources_info_retain")
                .expect("power sources info retain"),
        }
    }
}

/// Releases the retained power-sources snapshot handle on drop.
impl Drop for PowerSourcesInfo {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_iops_power_sources_info_release(self.as_ptr()) };
    }
}

/// Wraps `IOPSCopyExternalPowerAdapterDetails`.
pub fn copy_external_power_adapter_details() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_iops_copy_external_power_adapter_details().cast()) }
}

/// Wraps `IOPSGetTimeRemainingEstimate`.
pub fn time_remaining_estimate() -> f64 {
    unsafe { bridge::iokit_swift_iops_get_time_remaining_estimate() }
}

/// Wraps `IOPSGetBatteryWarningLevel`.
pub fn battery_warning_level() -> BatteryWarningLevel {
    BatteryWarningLevel::from_raw(unsafe { bridge::iokit_swift_iops_get_battery_warning_level() })
}
