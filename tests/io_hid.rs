use iokit::prelude::*;

#[test]
fn creates_hid_manager_and_snapshots_devices() -> iokit::Result<()> {
    let manager = HidManager::create(HID_MANAGER_OPTION_NONE)?;
    assert_ne!(HidManager::type_id(), 0);
    manager.open(HID_MANAGER_OPTION_NONE)?;

    let devices = manager.devices();
    if let Some(device) = devices.first() {
        assert_ne!(HidDevice::type_id(), 0);
        let _ = device.service();
        let _ = device.property("Product")?;
        let _ = device.all_matching_elements();

        if device.open(HID_MANAGER_OPTION_NONE).is_ok() {
            let _ = device.close(HID_MANAGER_OPTION_NONE);
        }
    }

    manager.close(HID_MANAGER_OPTION_NONE)?;
    Ok(())
}
