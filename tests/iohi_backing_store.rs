use iokit::prelude::*;

#[test]
fn reports_private_sdk_status() -> iokit::Result<()> {
    assert!(!public_sdk_available());
    assert!(unavailability_reason()?.contains("private IOKit class"));
    Ok(())
}
