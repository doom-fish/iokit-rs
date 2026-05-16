use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let manager = HidManager::create(HID_MANAGER_OPTION_NONE)?;
    let device_count = manager.devices().len();

    println!("hid manager type id = {}", HidManager::type_id());
    println!("device count = {device_count}");

    if let Some(device) = manager.devices().into_iter().next() {
        println!("hid device type id = {}", HidDevice::type_id());
        if let Some(service) = device.service() {
            println!("first HID service class = {}", service.class_name()?);
        }
    }

    Ok(())
}
