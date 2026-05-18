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
    io_connect::Connect,
    io_message::{power_message_from_raw, PowerMessage},
    object::{c_string, io_result, nonnull},
    CFValue,
};
use core::ffi::c_void;
use std::{collections::BTreeMap, ptr::NonNull};

pub const ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP: &str =
    ffi_impl::K_IOPMAssertPreventUserIdleSystemSleep;
pub const ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP: &str =
    ffi_impl::K_IOPMAssertPreventUserIdleDisplaySleep;
pub const ASSERT_PREVENT_DISK_IDLE: &str = ffi_impl::K_IOPMAssertPreventDiskIdle;
pub const ASSERT_NETWORK_CLIENT_ACTIVE: &str = ffi_impl::K_IOPMAssertNetworkClientActive;
pub const AUTO_WAKE: &str = ffi_impl::K_IOPMAutoWake;
pub const AUTO_POWER_ON: &str = ffi_impl::K_IOPMAutoPowerOn;
pub const AUTO_WAKE_OR_POWER_ON: &str = ffi_impl::K_IOPMAutoWakeOrPowerOn;
pub const AUTO_SLEEP: &str = ffi_impl::K_IOPMAutoSleep;
pub const AUTO_SHUTDOWN: &str = ffi_impl::K_IOPMAutoShutdown;
pub const AUTO_RESTART: &str = ffi_impl::K_IOPMAutoRestart;
pub const POWER_EVENT_TIME_KEY: &str = ffi_impl::K_IOPMPowerEventTimeKey;
pub const POWER_EVENT_APP_NAME_KEY: &str = ffi_impl::K_IOPMPowerEventAppNameKey;
pub const POWER_EVENT_TYPE_KEY: &str = ffi_impl::K_IOPMPowerEventTypeKey;
pub const CPU_POWER_LIMIT_PROCESSOR_SPEED_KEY: &str =
    ffi_impl::K_IOPMCPUPowerLimitProcessorSpeedKey;
pub const CPU_POWER_LIMIT_PROCESSOR_COUNT_KEY: &str =
    ffi_impl::K_IOPMCPUPowerLimitProcessorCountKey;
pub const CPU_POWER_LIMIT_SCHEDULER_TIME_KEY: &str = ffi_impl::K_IOPMCPUPowerLimitSchedulerTimeKey;
pub const ASSERTION_TIMEOUT_KEY: &str = ffi_impl::K_IOPMAssertionTimeoutKey;
pub const ASSERTION_TIMEOUT_ACTION_KEY: &str = ffi_impl::K_IOPMAssertionTimeoutActionKey;
pub const ASSERTION_TIMEOUT_ACTION_LOG: &str = ffi_impl::K_IOPMAssertionTimeoutActionLog;
pub const ASSERTION_TIMEOUT_ACTION_TURN_OFF: &str = ffi_impl::K_IOPMAssertionTimeoutActionTurnOff;
pub const ASSERTION_TIMEOUT_ACTION_RELEASE: &str = ffi_impl::K_IOPMAssertionTimeoutActionRelease;
pub const ASSERTION_RETAIN_COUNT_KEY: &str = ffi_impl::K_IOPMAssertionRetainCountKey;
pub const ASSERTION_NAME_KEY: &str = ffi_impl::K_IOPMAssertionNameKey;
pub const ASSERTION_DETAILS_KEY: &str = ffi_impl::K_IOPMAssertionDetailsKey;
pub const ASSERTION_HUMAN_READABLE_REASON_KEY: &str =
    ffi_impl::K_IOPMAssertionHumanReadableReasonKey;
pub const ASSERTION_LOCALIZATION_BUNDLE_PATH_KEY: &str =
    ffi_impl::K_IOPMAssertionLocalizationBundlePathKey;
pub const ASSERTION_FRAMEWORK_ID_KEY: &str = ffi_impl::K_IOPMAssertionFrameworkIDKey;
pub const ASSERTION_PLUGIN_ID_KEY: &str = ffi_impl::K_IOPMAssertionPlugInIDKey;
pub const ASSERTION_TYPE_KEY: &str = ffi_impl::K_IOPMAssertionTypeKey;
pub const ASSERTION_LEVEL_KEY: &str = ffi_impl::K_IOPMAssertionLevelKey;
pub const SYSTEM_LOAD_ADVISORY_NOTIFY_NAME: &str = ffi_impl::K_IOSystemLoadAdvisoryNotifyName;
pub const SYSTEM_LOAD_ADVISORY_USER_LEVEL_KEY: &str = ffi_impl::K_IOSystemLoadAdvisoryUserLevelKey;
pub const SYSTEM_LOAD_ADVISORY_BATTERY_LEVEL_KEY: &str =
    ffi_impl::K_IOSystemLoadAdvisoryBatteryLevelKey;
pub const SYSTEM_LOAD_ADVISORY_THERMAL_LEVEL_KEY: &str =
    ffi_impl::K_IOSystemLoadAdvisoryThermalLevelKey;
pub const SYSTEM_LOAD_ADVISORY_COMBINED_LEVEL_KEY: &str =
    ffi_impl::K_IOSystemLoadAdvisoryCombinedLevelKey;
pub const CPU_POWER_NOTIFICATION_KEY: &str = ffi_impl::K_IOPMCPUPowerNotificationKey;
pub const THERMAL_WARNING_NOTIFICATION_KEY: &str = ffi_impl::K_IOPMThermalWarningNotificationKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerAssertionLevel {
    Off,
    On,
}

impl PowerAssertionLevel {
    const fn as_raw(self) -> u32 {
        match self {
            Self::Off => ffi_impl::kIOPMAssertionLevelOff,
            Self::On => ffi_impl::kIOPMAssertionLevelOn,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserActiveType {
    Local,
    Remote,
}

impl UserActiveType {
    const fn as_raw(self) -> u32 {
        match self {
            Self::Local => ffi_impl::kIOPMUserActiveLocal,
            Self::Remote => ffi_impl::kIOPMUserActiveRemote,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ThermalWarningLevel {
    Normal,
    Danger,
    Crisis,
    Unknown(u32),
}

impl ThermalWarningLevel {
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            ffi_impl::kIOPMThermalWarningLevelNormal => Self::Normal,
            ffi_impl::kIOPMThermalWarningLevelDanger => Self::Danger,
            ffi_impl::kIOPMThermalWarningLevelCrisis => Self::Crisis,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SystemLoadAdvisoryLevel {
    Bad,
    Ok,
    Great,
    Unknown(i32),
}

impl SystemLoadAdvisoryLevel {
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            ffi_impl::kIOSystemLoadAdvisoryLevelBad => Self::Bad,
            ffi_impl::kIOSystemLoadAdvisoryLevelOK => Self::Ok,
            ffi_impl::kIOSystemLoadAdvisoryLevelGreat => Self::Great,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug)]
pub struct PowerAssertion {
    raw: NonNull<c_void>,
}

impl PowerAssertion {
    const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    pub fn create(assertion_type: &str, name: &str) -> Result<Self> {
        Self::create_with_level(assertion_type, PowerAssertionLevel::On, name)
    }

    pub fn create_with_level(
        assertion_type: &str,
        level: PowerAssertionLevel,
        name: &str,
    ) -> Result<Self> {
        let assertion_type = c_string(assertion_type)?;
        let name = c_string(name)?;
        let raw = unsafe {
            bridge::iokit_swift_power_assertion_create(
                assertion_type.as_ptr(),
                level.as_raw(),
                name.as_ptr(),
            )
        };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_power_assertion_create")?,
        })
    }

    pub fn declare_user_activity(name: &str, user_type: UserActiveType) -> Result<Self> {
        let name = c_string(name)?;
        let raw = unsafe {
            bridge::iokit_swift_power_assertion_declare_user_activity(
                name.as_ptr(),
                user_type.as_raw(),
            )
        };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_power_assertion_declare_user_activity")?,
        })
    }

    pub fn declare_network_client_activity(name: &str) -> Result<Self> {
        let name = c_string(name)?;
        let raw = unsafe {
            bridge::iokit_swift_power_assertion_declare_network_client_activity(name.as_ptr())
        };
        Ok(Self {
            raw: nonnull(
                raw,
                "iokit_swift_power_assertion_declare_network_client_activity",
            )?,
        })
    }

    pub fn id(&self) -> u32 {
        unsafe { bridge::iokit_swift_power_assertion_id(self.as_ptr()) }
    }

    pub fn properties(&self) -> Option<CFValue> {
        unsafe {
            take_value(bridge::iokit_swift_power_assertion_copy_properties(self.as_ptr()).cast())
        }
    }
}

impl Clone for PowerAssertion {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_power_assertion_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_power_assertion_retain")
                .expect("power assertion retain"),
        }
    }
}

impl Drop for PowerAssertion {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_power_assertion_release(self.as_ptr()) };
    }
}

pub fn find_power_management() -> Result<Connect> {
    Connect::from_raw(unsafe { bridge::iokit_swift_power_find_power_management() }).ok_or(
        crate::IoKitError::UnexpectedNull("iokit_swift_power_find_power_management"),
    )
}

pub fn sleep_enabled() -> bool {
    unsafe { bridge::iokit_swift_power_sleep_enabled() }
}

pub fn get_aggressiveness(connect: &Connect, aggressiveness_type: u64) -> Result<u64> {
    let mut value = 0_u64;
    io_result(
        unsafe {
            bridge::iokit_swift_power_get_aggressiveness(
                connect.as_ptr(),
                aggressiveness_type,
                &mut value,
            )
        },
        "IOPMGetAggressiveness",
    )?;
    Ok(value)
}

pub fn set_aggressiveness(connect: &Connect, aggressiveness_type: u64, value: u64) -> Result<()> {
    io_result(
        unsafe {
            bridge::iokit_swift_power_set_aggressiveness(
                connect.as_ptr(),
                aggressiveness_type,
                value,
            )
        },
        "IOPMSetAggressiveness",
    )
}

pub fn thermal_warning_level() -> Result<ThermalWarningLevel> {
    let mut level = 0_u32;
    io_result(
        unsafe { bridge::iokit_swift_power_get_thermal_warning_level(&mut level) },
        "IOPMGetThermalWarningLevel",
    )?;
    Ok(ThermalWarningLevel::from_raw(level))
}

pub fn system_load_advisory() -> SystemLoadAdvisoryLevel {
    SystemLoadAdvisoryLevel::from_raw(unsafe { ffi_impl::IOGetSystemLoadAdvisory() })
}

pub fn copy_system_load_advisory_detailed() -> Option<CFValue> {
    unsafe { take_value(ffi_impl::IOCopySystemLoadAdvisoryDetailed().cast()) }
}

pub fn copy_assertions_by_process() -> Result<CFValue> {
    Ok(
        unsafe { take_value(bridge::iokit_swift_power_copy_assertions_by_process().cast()) }
            .unwrap_or_else(|| CFValue::Dictionary(BTreeMap::default())),
    )
}

pub fn copy_assertions_status() -> Result<CFValue> {
    Ok(
        unsafe { take_value(bridge::iokit_swift_power_copy_assertions_status().cast()) }
            .unwrap_or_else(|| CFValue::Dictionary(BTreeMap::default())),
    )
}

pub fn copy_battery_info() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_power_copy_battery_info().cast()) }
}

pub fn copy_cpu_power_status() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_power_copy_cpu_power_status().cast()) }
}

pub fn copy_scheduled_power_events() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_power_copy_scheduled_power_events().cast()) }
}

pub const fn power_message(raw: u32) -> PowerMessage {
    power_message_from_raw(raw)
}
