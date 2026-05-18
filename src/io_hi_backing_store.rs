#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{bridge, error::Result, object::take_required_c_string};

/// Reports whether the public macOS SDK declares `IOHIBackingStore`.
pub fn public_sdk_available() -> bool {
    unsafe { bridge::iokit_swift_iohi_backing_store_public_sdk_available() }
}

/// Returns the public-SDK reason that `IOHIBackingStore` is unavailable.
pub fn unavailability_reason() -> Result<String> {
    unsafe {
        take_required_c_string(
            bridge::iokit_swift_iohi_backing_store_unavailability_reason(),
            "iokit_swift_iohi_backing_store_unavailability_reason",
        )
    }
}
