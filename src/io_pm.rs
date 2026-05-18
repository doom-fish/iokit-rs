//! Safe wrappers around `IOPMLib.h` power-management APIs.

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

/// Wraps `K_IOPMAssertPreventUserIdleSystemSleep`.
pub const ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP: &str =
    ffi_impl::K_IOPMAssertPreventUserIdleSystemSleep;
/// Wraps `K_IOPMAssertPreventUserIdleDisplaySleep`.
pub const ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP: &str =
    ffi_impl::K_IOPMAssertPreventUserIdleDisplaySleep;
/// Wraps `K_IOPMAssertPreventDiskIdle`.
pub const ASSERT_PREVENT_DISK_IDLE: &str = ffi_impl::K_IOPMAssertPreventDiskIdle;
/// Wraps `K_IOPMAssertNetworkClientActive`.
pub const ASSERT_NETWORK_CLIENT_ACTIVE: &str = ffi_impl::K_IOPMAssertNetworkClientActive;
/// Wraps `K_IOPMAutoWake`.
pub const AUTO_WAKE: &str = ffi_impl::K_IOPMAutoWake;
/// Wraps `K_IOPMAutoPowerOn`.
pub const AUTO_POWER_ON: &str = ffi_impl::K_IOPMAutoPowerOn;
/// Wraps `K_IOPMAutoWakeOrPowerOn`.
pub const AUTO_WAKE_OR_POWER_ON: &str = ffi_impl::K_IOPMAutoWakeOrPowerOn;
/// Wraps `K_IOPMAutoSleep`.
pub const AUTO_SLEEP: &str = ffi_impl::K_IOPMAutoSleep;
/// Wraps `K_IOPMAutoShutdown`.
pub const AUTO_SHUTDOWN: &str = ffi_impl::K_IOPMAutoShutdown;
/// Wraps `K_IOPMAutoRestart`.
pub const AUTO_RESTART: &str = ffi_impl::K_IOPMAutoRestart;
/// Wraps `K_IOPMPowerEventTimeKey`.
pub const POWER_EVENT_TIME_KEY: &str = ffi_impl::K_IOPMPowerEventTimeKey;
/// Wraps `K_IOPMPowerEventAppNameKey`.
pub const POWER_EVENT_APP_NAME_KEY: &str = ffi_impl::K_IOPMPowerEventAppNameKey;
/// Wraps `K_IOPMPowerEventTypeKey`.
pub const POWER_EVENT_TYPE_KEY: &str = ffi_impl::K_IOPMPowerEventTypeKey;
/// Wraps `K_IOPMCPUPowerLimitProcessorSpeedKey`.
pub const CPU_POWER_LIMIT_PROCESSOR_SPEED_KEY: &str =
    ffi_impl::K_IOPMCPUPowerLimitProcessorSpeedKey;
/// Wraps `K_IOPMCPUPowerLimitProcessorCountKey`.
pub const CPU_POWER_LIMIT_PROCESSOR_COUNT_KEY: &str =
    ffi_impl::K_IOPMCPUPowerLimitProcessorCountKey;
/// Wraps `K_IOPMCPUPowerLimitSchedulerTimeKey`.
pub const CPU_POWER_LIMIT_SCHEDULER_TIME_KEY: &str = ffi_impl::K_IOPMCPUPowerLimitSchedulerTimeKey;
/// Wraps `K_IOPMAssertionTimeoutKey`.
pub const ASSERTION_TIMEOUT_KEY: &str = ffi_impl::K_IOPMAssertionTimeoutKey;
/// Wraps `K_IOPMAssertionTimeoutActionKey`.
pub const ASSERTION_TIMEOUT_ACTION_KEY: &str = ffi_impl::K_IOPMAssertionTimeoutActionKey;
/// Wraps `K_IOPMAssertionTimeoutActionLog`.
pub const ASSERTION_TIMEOUT_ACTION_LOG: &str = ffi_impl::K_IOPMAssertionTimeoutActionLog;
/// Wraps `K_IOPMAssertionTimeoutActionTurnOff`.
pub const ASSERTION_TIMEOUT_ACTION_TURN_OFF: &str = ffi_impl::K_IOPMAssertionTimeoutActionTurnOff;
/// Wraps `K_IOPMAssertionTimeoutActionRelease`.
pub const ASSERTION_TIMEOUT_ACTION_RELEASE: &str = ffi_impl::K_IOPMAssertionTimeoutActionRelease;
/// Wraps `K_IOPMAssertionRetainCountKey`.
pub const ASSERTION_RETAIN_COUNT_KEY: &str = ffi_impl::K_IOPMAssertionRetainCountKey;
/// Wraps `K_IOPMAssertionNameKey`.
pub const ASSERTION_NAME_KEY: &str = ffi_impl::K_IOPMAssertionNameKey;
/// Wraps `K_IOPMAssertionDetailsKey`.
pub const ASSERTION_DETAILS_KEY: &str = ffi_impl::K_IOPMAssertionDetailsKey;
/// Wraps `K_IOPMAssertionHumanReadableReasonKey`.
pub const ASSERTION_HUMAN_READABLE_REASON_KEY: &str =
    ffi_impl::K_IOPMAssertionHumanReadableReasonKey;
/// Wraps `K_IOPMAssertionLocalizationBundlePathKey`.
pub const ASSERTION_LOCALIZATION_BUNDLE_PATH_KEY: &str =
    ffi_impl::K_IOPMAssertionLocalizationBundlePathKey;
/// Wraps `K_IOPMAssertionFrameworkIDKey`.
pub const ASSERTION_FRAMEWORK_ID_KEY: &str = ffi_impl::K_IOPMAssertionFrameworkIDKey;
/// Wraps `K_IOPMAssertionPlugInIDKey`.
pub const ASSERTION_PLUGIN_ID_KEY: &str = ffi_impl::K_IOPMAssertionPlugInIDKey;
/// Wraps `K_IOPMAssertionTypeKey`.
pub const ASSERTION_TYPE_KEY: &str = ffi_impl::K_IOPMAssertionTypeKey;
/// Wraps `K_IOPMAssertionLevelKey`.
pub const ASSERTION_LEVEL_KEY: &str = ffi_impl::K_IOPMAssertionLevelKey;
/// Wraps `K_IOSystemLoadAdvisoryNotifyName`.
pub const SYSTEM_LOAD_ADVISORY_NOTIFY_NAME: &str = ffi_impl::K_IOSystemLoadAdvisoryNotifyName;
/// Wraps `K_IOSystemLoadAdvisoryUserLevelKey`.
pub const SYSTEM_LOAD_ADVISORY_USER_LEVEL_KEY: &str = ffi_impl::K_IOSystemLoadAdvisoryUserLevelKey;
/// Wraps `K_IOSystemLoadAdvisoryBatteryLevelKey`.
pub const SYSTEM_LOAD_ADVISORY_BATTERY_LEVEL_KEY: &str =
    ffi_impl::K_IOSystemLoadAdvisoryBatteryLevelKey;
/// Wraps `K_IOSystemLoadAdvisoryThermalLevelKey`.
pub const SYSTEM_LOAD_ADVISORY_THERMAL_LEVEL_KEY: &str =
    ffi_impl::K_IOSystemLoadAdvisoryThermalLevelKey;
/// Wraps `K_IOSystemLoadAdvisoryCombinedLevelKey`.
pub const SYSTEM_LOAD_ADVISORY_COMBINED_LEVEL_KEY: &str =
    ffi_impl::K_IOSystemLoadAdvisoryCombinedLevelKey;
/// Wraps `K_IOPMCPUPowerNotificationKey`.
pub const CPU_POWER_NOTIFICATION_KEY: &str = ffi_impl::K_IOPMCPUPowerNotificationKey;
/// Wraps `K_IOPMThermalWarningNotificationKey`.
pub const THERMAL_WARNING_NOTIFICATION_KEY: &str = ffi_impl::K_IOPMThermalWarningNotificationKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Typed wrapper around `IOPMAssertionLevel` values.
pub enum PowerAssertionLevel {
    /// Wraps `kIOPMAssertionLevelOff`.
    Off,
    /// Wraps `kIOPMAssertionLevelOn`.
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
/// Typed wrapper around user-activity kinds accepted by power assertions.
pub enum UserActiveType {
    /// Wraps `kIOPMUserActiveLocal`.
    Local,
    /// Wraps `kIOPMUserActiveRemote`.
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
/// Typed wrapper around `kIOPMThermalWarningLevel*` values.
pub enum ThermalWarningLevel {
    /// Wraps `kIOPMThermalWarningLevelNormal`.
    Normal,
    /// Wraps `kIOPMThermalWarningLevelDanger`.
    Danger,
    /// Wraps `kIOPMThermalWarningLevelCrisis`.
    Crisis,
    /// Wraps an unrecognized thermal warning level.
    Unknown(u32),
}

impl ThermalWarningLevel {
    /// Builds a `ThermalWarningLevel` from a raw warning code.
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
/// Typed wrapper around `IOSystemLoadAdvisoryLevel` values.
pub enum SystemLoadAdvisoryLevel {
    /// Wraps `kIOSystemLoadAdvisoryLevelBad`.
    Bad,
    /// Wraps `kIOSystemLoadAdvisoryLevelOK`.
    Ok,
    /// Wraps `kIOSystemLoadAdvisoryLevelGreat`.
    Great,
    /// Wraps an unrecognized system load advisory level.
    Unknown(i32),
}

impl SystemLoadAdvisoryLevel {
    /// Builds a `SystemLoadAdvisoryLevel` from a raw advisory code.
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
/// Safe retained wrapper around a Swift-owned power assertion object.
pub struct PowerAssertion {
    raw: NonNull<c_void>,
}

impl PowerAssertion {
    const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    /// Creates an enabled power assertion with the given type and name.
    pub fn create(assertion_type: &str, name: &str) -> Result<Self> {
        Self::create_with_level(assertion_type, PowerAssertionLevel::On, name)
    }

    /// Creates a power assertion with an explicit assertion level.
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

    /// Wraps `IOPMAssertionDeclareUserActivity`.
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

    /// Wraps the network-client activity assertion helper.
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

    /// Returns the current assertion identifier.
    pub fn id(&self) -> u32 {
        unsafe { bridge::iokit_swift_power_assertion_id(self.as_ptr()) }
    }

    /// Copies the current assertion properties dictionary.
    pub fn properties(&self) -> Option<CFValue> {
        unsafe {
            take_value(bridge::iokit_swift_power_assertion_copy_properties(self.as_ptr()).cast())
        }
    }
}

/// Clones the retained power-assertion handle.
impl Clone for PowerAssertion {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_power_assertion_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_power_assertion_retain")
                .expect("power assertion retain"),
        }
    }
}

/// Releases the retained power-assertion handle on drop.
impl Drop for PowerAssertion {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_power_assertion_release(self.as_ptr()) };
    }
}

/// Wraps `IOPMFindPowerManagement`.
pub fn find_power_management() -> Result<Connect> {
    Connect::from_raw(unsafe { bridge::iokit_swift_power_find_power_management() }).ok_or(
        crate::IoKitError::UnexpectedNull("iokit_swift_power_find_power_management"),
    )
}

/// Wraps `IOPMSleepEnabled`.
pub fn sleep_enabled() -> bool {
    unsafe { bridge::iokit_swift_power_sleep_enabled() }
}

/// Wraps `IOPMGetAggressiveness`.
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

/// Wraps `IOPMSetAggressiveness`.
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

/// Wraps `IOPMGetThermalWarningLevel`.
pub fn thermal_warning_level() -> Result<ThermalWarningLevel> {
    let mut level = 0_u32;
    io_result(
        unsafe { bridge::iokit_swift_power_get_thermal_warning_level(&mut level) },
        "IOPMGetThermalWarningLevel",
    )?;
    Ok(ThermalWarningLevel::from_raw(level))
}

/// Wraps `IOGetSystemLoadAdvisory`.
pub fn system_load_advisory() -> SystemLoadAdvisoryLevel {
    SystemLoadAdvisoryLevel::from_raw(unsafe { ffi_impl::IOGetSystemLoadAdvisory() })
}

/// Wraps `IOCopySystemLoadAdvisoryDetailed`.
pub fn copy_system_load_advisory_detailed() -> Option<CFValue> {
    unsafe { take_value(ffi_impl::IOCopySystemLoadAdvisoryDetailed().cast()) }
}

/// Wraps `IOPMCopyAssertionsByProcess`.
pub fn copy_assertions_by_process() -> Result<CFValue> {
    Ok(
        unsafe { take_value(bridge::iokit_swift_power_copy_assertions_by_process().cast()) }
            .unwrap_or_else(|| CFValue::Dictionary(BTreeMap::default())),
    )
}

/// Wraps `IOPMCopyAssertionsStatus`.
pub fn copy_assertions_status() -> Result<CFValue> {
    Ok(
        unsafe { take_value(bridge::iokit_swift_power_copy_assertions_status().cast()) }
            .unwrap_or_else(|| CFValue::Dictionary(BTreeMap::default())),
    )
}

/// Wraps `IOPMCopyBatteryInfo`.
pub fn copy_battery_info() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_power_copy_battery_info().cast()) }
}

/// Wraps `IOPMCopyCPUPowerStatus`.
pub fn copy_cpu_power_status() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_power_copy_cpu_power_status().cast()) }
}

/// Wraps `IOPMCopyScheduledPowerEvents`.
pub fn copy_scheduled_power_events() -> Option<CFValue> {
    unsafe { take_value(bridge::iokit_swift_power_copy_scheduled_power_events().cast()) }
}

/// Builds a `PowerMessage` from a raw power callback code.
pub const fn power_message(raw: u32) -> PowerMessage {
    power_message_from_raw(raw)
}
