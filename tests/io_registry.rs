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

#[test]
fn set_property_reports_errors_instead_of_failing_silently() {
    use iokit::{CFValue, IoKitError};

    let root = root_registry_entry().expect("registry root");
    assert!(matches!(
        root.set_property("fish.doom.iokit.test", &CFValue::Unknown(1)),
        Err(IoKitError::InvalidArgument(_))
    ));
    assert!(matches!(
        root.set_property("fish.doom.iokit.test", &CFValue::Integer(1)),
        Err(IoKitError::IoReturn("IORegistryEntrySetCFProperty", _))
    ));
    assert_eq!(root.property("fish.doom.iokit.test").expect("lookup"), None);
}
