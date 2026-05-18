#![doc = "Safe Rust bindings for Apple's `IOKit` user-space APIs on macOS."]
#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! This crate wraps user-space entry points from Apple's `IOKit` framework.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod bridge;
mod object;

/// Core Foundation snapshot helpers used by the safe `IOKit` wrappers.
pub mod cf;
/// Error types returned by the safe `IOKit` wrappers.
pub mod error;
#[path = "ffi/mod.rs"]
pub(crate) mod ffi_impl;
/// Safe wrappers around `IOCFSerialize`, `IOCFUnserialize`, and related helpers.
pub mod io_cf;
/// Safe wrappers around `io_connect_t` and the `IOConnect*` call family.
pub mod io_connect;
/// Public-SDK availability helpers for the `IOHIBackingStore` surface.
pub mod io_hi_backing_store;
/// Safe wrappers around `IOHIDManager` and `IOHIDDevice`.
pub mod io_hid;
/// Safe wrappers around `IOIteratorNext` and related iterator entry points.
pub mod io_iterator;
/// Safe wrappers around `IOKitLib.h` global entry points.
pub mod io_kit;
/// Typed enums for `IOMessage.h` constants.
pub mod io_message;
/// Safe wrappers around `IONotificationPort*` APIs.
pub mod io_notification_port;
/// Safe wrappers around `IOPMLib.h` power-management APIs.
pub mod io_pm;
/// Safe wrappers around `IORegistryEntry*` APIs.
pub mod io_registry;
/// Safe wrappers around `IOService*` matching and lookup APIs.
pub mod io_service;
/// Safe wrappers around `IOPS*` power-source APIs.
pub mod iops;
/// Power-assertion helpers built on `IOPMAssertion*` APIs.
pub mod power;
/// Registry convenience helpers built on `IORegistryEntry*` APIs.
pub mod registry;
/// Shared raw type aliases for `IOKit` handles and status codes.
pub mod types;

/// Raw `IOKit` and Core Foundation FFI re-exports from the audited `ffi_impl` surface.
#[cfg(feature = "raw-ffi")]
pub mod ffi {
    pub use crate::ffi_impl::*;
}

/// Async stream wrappers around notification-style `IOKit` callbacks.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;

pub use cf::CFValue;
pub use error::{IoKitError, Result};
pub use io_cf::{
    serialize_raw, unserialize, unserialize_binary, unserialize_with_size, SERIALIZE_TO_BINARY,
};
pub use io_connect::{Connect, ConnectCallOutput};
pub use io_hi_backing_store::{public_sdk_available, unavailability_reason};
pub use io_hid::{
    HidDevice, HidManager, HidReportType, HID_DEVICE_GET_VALUE_WITHOUT_UPDATE,
    HID_DEVICE_GET_VALUE_WITH_UPDATE, HID_MANAGER_OPTION_DO_NOT_LOAD_PROPERTIES,
    HID_MANAGER_OPTION_DO_NOT_SAVE_PROPERTIES, HID_MANAGER_OPTION_INDEPENDENT_DEVICES,
    HID_MANAGER_OPTION_NONE, HID_MANAGER_OPTION_USE_PERSISTENT_PROPERTIES,
};
pub use io_iterator::ObjectIterator;
pub use io_kit::{
    bsd_name_matching_service, bsd_name_matching_service_for_port, bsd_name_matching_services,
    bsd_name_matching_services_iterator, bsd_name_matching_services_iterator_for_port,
    catalogue_get_data_raw, catalogue_module_loaded, catalogue_reset, catalogue_send_data,
    catalogue_terminate, create_receive_port, dispatch_callout_from_message, kit_busy_state,
    kit_wait_quiet, main_port, main_port_from_bootstrap, registry_iterator,
    registry_iterator_for_port, root_registry_entry, root_registry_entry_for_port,
    CATALOG_ADD_DRIVERS, CATALOG_ADD_DRIVERS_NO_MATCH, CATALOG_GET_CACHE_MISS_LIST,
    CATALOG_GET_CONTENTS, CATALOG_GET_MODULE_DEMAND_LIST, CATALOG_GET_ROM_MKEXT_LIST,
    CATALOG_KEXTD_ACTIVE, CATALOG_KEXTD_FINISHED_LAUNCHING, CATALOG_MODULE_TERMINATE,
    CATALOG_MODULE_UNLOAD, CATALOG_REMOVE_DRIVERS, CATALOG_REMOVE_DRIVERS_NO_MATCH,
    CATALOG_RESET_DEFAULT, CATALOG_RESET_DRIVERS, CATALOG_RESET_DRIVERS_NO_MATCH,
    CATALOG_SERVICE_TERMINATE, MAIN_PORT_DEFAULT,
};
pub use io_message::{
    bridged_constant as io_message_bridged_constant,
    bridged_constant_count as io_message_bridged_constant_count, power_message_from_raw, IoMessage,
    PowerMessage,
};
pub use io_notification_port::NotificationPort;
pub use io_pm::{
    copy_assertions_by_process, copy_assertions_status, copy_battery_info, copy_cpu_power_status,
    copy_scheduled_power_events, copy_system_load_advisory_detailed, find_power_management,
    get_aggressiveness, power_message, set_aggressiveness, sleep_enabled, system_load_advisory,
    thermal_warning_level, PowerAssertion, PowerAssertionLevel, SystemLoadAdvisoryLevel,
    ThermalWarningLevel, UserActiveType, ASSERTION_DETAILS_KEY, ASSERTION_FRAMEWORK_ID_KEY,
    ASSERTION_HUMAN_READABLE_REASON_KEY, ASSERTION_LEVEL_KEY,
    ASSERTION_LOCALIZATION_BUNDLE_PATH_KEY, ASSERTION_NAME_KEY, ASSERTION_PLUGIN_ID_KEY,
    ASSERTION_RETAIN_COUNT_KEY, ASSERTION_TIMEOUT_ACTION_KEY, ASSERTION_TIMEOUT_ACTION_LOG,
    ASSERTION_TIMEOUT_ACTION_RELEASE, ASSERTION_TIMEOUT_ACTION_TURN_OFF, ASSERTION_TIMEOUT_KEY,
    ASSERTION_TYPE_KEY, ASSERT_NETWORK_CLIENT_ACTIVE, ASSERT_PREVENT_DISK_IDLE,
    ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP, ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP, AUTO_POWER_ON,
    AUTO_RESTART, AUTO_SHUTDOWN, AUTO_SLEEP, AUTO_WAKE, AUTO_WAKE_OR_POWER_ON,
    CPU_POWER_LIMIT_PROCESSOR_COUNT_KEY, CPU_POWER_LIMIT_PROCESSOR_SPEED_KEY,
    CPU_POWER_LIMIT_SCHEDULER_TIME_KEY, CPU_POWER_NOTIFICATION_KEY, POWER_EVENT_APP_NAME_KEY,
    POWER_EVENT_TIME_KEY, POWER_EVENT_TYPE_KEY, SYSTEM_LOAD_ADVISORY_BATTERY_LEVEL_KEY,
    SYSTEM_LOAD_ADVISORY_COMBINED_LEVEL_KEY, SYSTEM_LOAD_ADVISORY_NOTIFY_NAME,
    SYSTEM_LOAD_ADVISORY_THERMAL_LEVEL_KEY, SYSTEM_LOAD_ADVISORY_USER_LEVEL_KEY,
    THERMAL_WARNING_NOTIFICATION_KEY,
};
pub use io_registry::{RegistryEntry, REGISTRY_ITERATE_PARENTS, REGISTRY_ITERATE_RECURSIVELY};
pub use io_service::{
    matching_service, matching_service_entry_id, matching_services, matching_services_iterator,
    name_matching_service, name_matching_services, name_matching_services_iterator, Service,
    BUSY_INTEREST, FIRST_MATCH_NOTIFICATION, FIRST_PUBLISH_NOTIFICATION, GENERAL_INTEREST,
    MATCHED_NOTIFICATION, PUBLISH_NOTIFICATION, SERVICE_INTERACTION_ALLOWED, SERVICE_PLANE,
    TERMINATED_NOTIFICATION,
};
pub use iops::{
    battery_warning_level, copy_external_power_adapter_details, time_remaining_estimate,
    BatteryWarningLevel, PowerSourcesInfo, NOTIFY_ANY_POWER_SOURCE, NOTIFY_ATTACH,
    NOTIFY_LOW_BATTERY, NOTIFY_POWER_SOURCE, NOTIFY_TIME_REMAINING, POWER_SOURCES_NOTIFICATION_KEY,
    PROVIDING_POWER_AC, PROVIDING_POWER_BATTERY, PROVIDING_POWER_UPS,
    TIME_REMAINING_NOTIFICATION_KEY, TIME_REMAINING_UNKNOWN, TIME_REMAINING_UNLIMITED,
};

/// Common imports.
pub mod prelude {
    pub use crate::cf::CFValue;
    pub use crate::error::{IoKitError, Result};
    pub use crate::io_cf::{unserialize, unserialize_binary, unserialize_with_size};
    pub use crate::io_connect::{Connect, ConnectCallOutput};
    pub use crate::io_hi_backing_store::{public_sdk_available, unavailability_reason};
    pub use crate::io_hid::{
        HidDevice, HidManager, HidReportType, HID_MANAGER_OPTION_NONE,
        HID_MANAGER_OPTION_USE_PERSISTENT_PROPERTIES,
    };
    pub use crate::io_iterator::ObjectIterator;
    pub use crate::io_kit::{
        bsd_name_matching_service, bsd_name_matching_services, create_receive_port, kit_busy_state,
        kit_wait_quiet, main_port, registry_iterator, root_registry_entry, MAIN_PORT_DEFAULT,
    };
    pub use crate::io_message::{power_message_from_raw, IoMessage, PowerMessage};
    pub use crate::io_notification_port::NotificationPort;
    pub use crate::io_pm::{
        copy_assertions_by_process, copy_assertions_status, copy_battery_info,
        copy_cpu_power_status, copy_scheduled_power_events, copy_system_load_advisory_detailed,
        find_power_management, get_aggressiveness, power_message, set_aggressiveness,
        sleep_enabled, system_load_advisory, thermal_warning_level, PowerAssertion,
        PowerAssertionLevel, SystemLoadAdvisoryLevel, ThermalWarningLevel, UserActiveType,
        ASSERT_NETWORK_CLIENT_ACTIVE, ASSERT_PREVENT_DISK_IDLE,
        ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP, ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP,
        AUTO_POWER_ON, AUTO_RESTART, AUTO_SHUTDOWN, AUTO_SLEEP, AUTO_WAKE, AUTO_WAKE_OR_POWER_ON,
    };
    pub use crate::io_registry::{
        RegistryEntry, REGISTRY_ITERATE_PARENTS, REGISTRY_ITERATE_RECURSIVELY,
    };
    pub use crate::io_service::{
        matching_service, matching_service_entry_id, matching_services, matching_services_iterator,
        name_matching_service, name_matching_services, name_matching_services_iterator, Service,
        BUSY_INTEREST, FIRST_MATCH_NOTIFICATION, FIRST_PUBLISH_NOTIFICATION, GENERAL_INTEREST,
        MATCHED_NOTIFICATION, PUBLISH_NOTIFICATION, SERVICE_INTERACTION_ALLOWED, SERVICE_PLANE,
        TERMINATED_NOTIFICATION,
    };
    pub use crate::iops::{
        battery_warning_level, copy_external_power_adapter_details, time_remaining_estimate,
        BatteryWarningLevel, PowerSourcesInfo, NOTIFY_ANY_POWER_SOURCE, NOTIFY_ATTACH,
        NOTIFY_LOW_BATTERY, NOTIFY_POWER_SOURCE, NOTIFY_TIME_REMAINING,
        POWER_SOURCES_NOTIFICATION_KEY, PROVIDING_POWER_AC, PROVIDING_POWER_BATTERY,
        PROVIDING_POWER_UPS, TIME_REMAINING_NOTIFICATION_KEY, TIME_REMAINING_UNKNOWN,
        TIME_REMAINING_UNLIMITED,
    };
}
