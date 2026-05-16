use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    if let Some(entry) = RegistryEntry::from_path("IOService:/IOResources")? {
        let name = entry.name()?;
        let path = entry.path(SERVICE_PLANE)?;
        let io_class = entry.property("IOClass")?;
        println!("registry entry name = {name}");
        println!("registry entry path = {path}");
        println!("IOClass property = {io_class:?}");
    } else {
        println!("IOService:/IOResources entry was not found");
    }

    Ok(())
}
