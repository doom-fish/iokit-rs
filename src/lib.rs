#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's `IOKit` user-space APIs on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod bridge;
mod object;

pub mod cf;
pub mod error;
#[path = "ffi/mod.rs"]
pub(crate) mod ffi_impl;
pub mod io_connect;
pub mod io_hi_backing_store;
pub mod io_iterator;
pub mod io_message;
pub mod io_notification_port;
pub mod io_pm;
pub mod io_registry;
pub mod io_service;
pub mod iops;
pub mod power;
pub mod registry;
pub mod types;

#[cfg(feature = "raw-ffi")]
pub mod ffi {
    pub use crate::ffi_impl::*;
}

pub use cf::CFValue;
pub use error::{IoKitError, Result};
pub use io_connect::{Connect, ConnectCallOutput};
pub use io_hi_backing_store::{public_sdk_available, unavailability_reason};
pub use io_iterator::ObjectIterator;
pub use io_message::{
    bridged_constant as io_message_bridged_constant,
    bridged_constant_count as io_message_bridged_constant_count, power_message_from_raw, IoMessage,
    PowerMessage,
};
pub use io_notification_port::NotificationPort;
pub use io_pm::{
    copy_assertions_by_process, copy_assertions_status, copy_battery_info, copy_cpu_power_status,
    copy_scheduled_power_events, find_power_management, get_aggressiveness, power_message,
    set_aggressiveness, sleep_enabled, thermal_warning_level, PowerAssertion, PowerAssertionLevel,
    ThermalWarningLevel, UserActiveType, ASSERT_NETWORK_CLIENT_ACTIVE, ASSERT_PREVENT_DISK_IDLE,
    ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP, ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP, AUTO_POWER_ON,
    AUTO_RESTART, AUTO_SHUTDOWN, AUTO_SLEEP, AUTO_WAKE, AUTO_WAKE_OR_POWER_ON,
    CPU_POWER_LIMIT_PROCESSOR_COUNT_KEY, CPU_POWER_LIMIT_PROCESSOR_SPEED_KEY,
    CPU_POWER_LIMIT_SCHEDULER_TIME_KEY, POWER_EVENT_APP_NAME_KEY, POWER_EVENT_TIME_KEY,
    POWER_EVENT_TYPE_KEY,
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
    pub use crate::io_connect::{Connect, ConnectCallOutput};
    pub use crate::io_hi_backing_store::{public_sdk_available, unavailability_reason};
    pub use crate::io_iterator::ObjectIterator;
    pub use crate::io_message::{power_message_from_raw, IoMessage, PowerMessage};
    pub use crate::io_notification_port::NotificationPort;
    pub use crate::io_pm::{
        copy_assertions_by_process, copy_assertions_status, copy_battery_info,
        copy_cpu_power_status, copy_scheduled_power_events, find_power_management,
        get_aggressiveness, power_message, set_aggressiveness, sleep_enabled,
        thermal_warning_level, PowerAssertion, PowerAssertionLevel, ThermalWarningLevel,
        UserActiveType, ASSERT_NETWORK_CLIENT_ACTIVE, ASSERT_PREVENT_DISK_IDLE,
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
