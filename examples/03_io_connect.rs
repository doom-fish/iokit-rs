use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let connect = find_power_management()?;
    if let Some(service) = connect.get_service() {
        let class_name = service.class_name()?;
        println!("connected service class = {class_name}");
    } else {
        println!("power management connection did not expose a service");
    }

    Ok(())
}
