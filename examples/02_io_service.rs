use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    if let Some(service) = matching_service("IOResources")? {
        let class_name = service.class_name()?;
        let name = service.name()?;
        let entry_id = service.registry_entry_id()?;
        println!("service class = {class_name}");
        println!("service name = {name}");
        println!("registry entry id = {entry_id}");
    } else {
        println!("IOResources service was not found");
    }

    Ok(())
}
