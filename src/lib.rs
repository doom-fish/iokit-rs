#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's `IOKit` user-space APIs on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod object;

pub mod cf;
pub mod error;
pub mod ffi;
pub mod power;
pub mod registry;
pub mod types;

pub use cf::CFValue;
pub use error::{IoKitError, Result};
pub use power::{
    copy_assertions_by_process, PowerAssertion, PowerAssertionLevel,
    ASSERT_PREVENT_DISK_IDLE, ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP,
    ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP, ASSERT_NETWORK_CLIENT_ACTIVE,
};
pub use registry::{matching_service, matching_services, ObjectIterator, Service, SERVICE_PLANE};
pub use types::PowerMessage;

/// Common imports.
pub mod prelude {
    pub use crate::cf::CFValue;
    pub use crate::error::{IoKitError, Result};
    pub use crate::power::{
        copy_assertions_by_process, PowerAssertion, PowerAssertionLevel,
        ASSERT_PREVENT_DISK_IDLE, ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP,
        ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP, ASSERT_NETWORK_CLIENT_ACTIVE,
    };
    pub use crate::registry::{
        matching_service, matching_services, ObjectIterator, Service, SERVICE_PLANE,
    };
    pub use crate::types::PowerMessage;
}
