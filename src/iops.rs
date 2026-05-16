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

pub const NOTIFY_LOW_BATTERY: &str = ffi_impl::K_IOPSNotifyLowBattery;
pub const NOTIFY_TIME_REMAINING: &str = ffi_impl::K_IOPSNotifyTimeRemaining;
pub const NOTIFY_POWER_SOURCE: &str = ffi_impl::K_IOPSNotifyPowerSource;
pub const NOTIFY_ATTACH: &str = ffi_impl::K_IOPSNotifyAttach;
pub const NOTIFY_ANY_POWER_SOURCE: &str = ffi_impl::K_IOPSNotifyAnyPowerSource;
pub const POWER_SOURCES_NOTIFICATION_KEY: &str = ffi_impl::K_IOPSPowerSourcesNotificationKey;
pub const TIME_REMAINING_NOTIFICATION_KEY: &str = ffi_impl::K_IOPSTimeRemainingNotificationKey;
pub const PROVIDING_POWER_AC: &str = ffi_impl::K_IOPMACPowerKey;
pub const PROVIDING_POWER_BATTERY: &str = ffi_impl::K_IOPMBatteryPowerKey;
pub const PROVIDING_POWER_UPS: &str = ffi_impl::K_IOPMUPSPowerKey;
pub const TIME_REMAINING_UNKNOWN: f64 = ffi_impl::kIOPSTimeRemainingUnknown;
pub const TIME_REMAINING_UNLIMITED: f64 = ffi_impl::kIOPSTimeRemainingUnlimited;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BatteryWarningLevel {
    None,
    Early,
    Final,
    Unknown(u32),
}

impl BatteryWarningLevel {
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
pub struct PowerSourcesInfo {
    raw: NonNull<c_void>,
}

impl PowerSourcesInfo {
    pub fn copy() -> Result<Self> {
        let raw = unsafe { bridge::iokit_swift_iops_power_sources_info_create() };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_iops_power_sources_info_create")?,
        })
    }

    const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn len(&self) -> usize {
        unsafe { bridge::iokit_swift_iops_power_sources_info_count(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn description(&self, index: usize) -> Option<CFValue> {
        unsafe {
            take_value(
                bridge::iokit_swift_iops_power_sources_info_description(self.as_ptr(), index)
                    .cast(),
            )
        }
    }

    pub fn providing_power_source_type(&self) -> Option<String> {
        unsafe {
            take_c_string(bridge::iokit_swift_iops_power_sources_info_provider_type(
                self.as_ptr(),
            ))
        }
    }
}

impl Clone for PowerSourcesInfo {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_iops_power_sources_info_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_iops_power_sources_info_retain")
                .expect("power sources info retain"),
        }
    }
}

impl Drop for PowerSourcesInfo {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_iops_power_sources_info_release(self.as_ptr()) };
    }
}

pub fn copy_external_power_adapter_details() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_iops_copy_external_power_adapter_details().cast()) }
}

pub fn time_remaining_estimate() -> f64 {
    unsafe { bridge::iokit_swift_iops_get_time_remaining_estimate() }
}

pub fn battery_warning_level() -> BatteryWarningLevel {
    BatteryWarningLevel::from_raw(unsafe { bridge::iokit_swift_iops_get_battery_warning_level() })
}
