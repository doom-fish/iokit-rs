use iokit::prelude::*;

#[test]
fn creates_notification_port() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    assert_ne!(port.mach_port(), 0);
    assert_ne!(port.run_loop_source_raw(), 0);
    Ok(())
}
