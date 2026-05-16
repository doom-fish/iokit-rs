use iokit::prelude::*;

#[test]
fn queries_main_port_and_registry_root() -> iokit::Result<()> {
    let main_port = main_port()?;
    let _busy_state = kit_busy_state(main_port)?;

    if let Err(error) = kit_wait_quiet(main_port, Some(1)) {
        assert!(error.to_string().contains("IOKitWaitQuiet"));
    }

    let Some(root) = root_registry_entry() else {
        panic!("IORegistryGetRootEntry returned MACH_PORT_NULL");
    };
    assert!(!root.name()?.is_empty());

    let mut iterator = registry_iterator(SERVICE_PLANE, 0)?
        .expect("IORegistryCreateIterator returned a null iterator handle");
    if iterator.next_registry_entry().is_some() {
        iterator.enter_entry()?;
        iterator.exit_entry()?;
    }

    let _ = bsd_name_matching_service("disk0")?;
    Ok(())
}
