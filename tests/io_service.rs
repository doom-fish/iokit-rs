use iokit::prelude::*;

#[test]
fn matches_ioresources_service() -> iokit::Result<()> {
    let Some(service) = matching_service("IOResources")? else {
        panic!("IOResources service was not found");
    };

    let class_name = service.class_name()?;
    assert_eq!(class_name, "IOResources");
    assert!(service.path(SERVICE_PLANE)?.contains("IOResources"));
    assert!(matching_service_entry_id(service.registry_entry_id()?).is_some());
    Ok(())
}
