use iokit::prelude::*;

#[test]
fn looks_up_registry_entry_by_path() -> iokit::Result<()> {
    let Some(entry) = RegistryEntry::from_path("IOService:/IOResources")? else {
        panic!("IOService:/IOResources registry entry was not found");
    };

    assert_eq!(entry.name()?, "IOResources");
    assert!(entry.in_plane(SERVICE_PLANE)?);
    assert!(entry.path(SERVICE_PLANE)?.contains("IOResources"));
    Ok(())
}
