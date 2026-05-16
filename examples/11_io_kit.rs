use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let main_port = main_port()?;
    let busy_state = kit_busy_state(main_port)?;
    let root_name = root_registry_entry().and_then(|entry| entry.name().ok());

    println!("main port = {main_port}");
    println!("global busy state = {busy_state}");
    println!("registry root = {}", root_name.unwrap_or_else(|| "<unknown>".to_string()));

    if let Some(service) = bsd_name_matching_service("disk0")? {
        println!("disk0 service class = {}", service.class_name()?);
    } else {
        println!("disk0 service = <not found>");
    }

    Ok(())
}
