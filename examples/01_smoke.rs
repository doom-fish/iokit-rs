use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let service = matching_service("IOResources")?
        .or_else(|| matching_service("IOPlatformExpertDevice").ok().flatten());

    if let Some(service) = service {
        println!("matched class: {}", service.class_name()?);
        println!("io class property: {:?}", service.property("IOClass")?);
        println!(
            "child count on IOService plane: {}",
            service.children(SERVICE_PLANE)?.len()
        );
    } else {
        println!("matched class: <none>");
    }

    let assertions = copy_assertions_by_process()?;
    println!("assertions snapshot kind: {}", assertions.kind_name());
    println!("✅ iokit registry + power OK");
    Ok(())
}
