use iokit::prelude::*;

#[test]
fn creates_notification_port() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    assert_ne!(port.mach_port(), 0);
    let source = port.run_loop_source_raw();
    assert_ne!(source, 0);
    let type_id = unsafe { iokit::ffi::CFGetTypeID(source as iokit::ffi::CFTypeRef) };
    assert_eq!(type_id, unsafe {
        apple_cf::raw::CFRunLoopSourceGetTypeID()
    });
    assert_eq!(port.run_loop_source_raw(), source);
    Ok(())
}
