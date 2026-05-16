use iokit::prelude::*;

#[test]
fn reads_power_management_snapshots() -> iokit::Result<()> {
    let assertions = copy_assertions_by_process()?;
    assert_eq!(assertions.kind_name(), "Dictionary");

    let connect = find_power_management()?;
    let _service = connect.get_service();

    match thermal_warning_level() {
        Ok(level) => assert!(matches!(
            level,
            ThermalWarningLevel::Normal
                | ThermalWarningLevel::Danger
                | ThermalWarningLevel::Crisis
                | ThermalWarningLevel::Unknown(_)
        )),
        Err(error) => assert!(error.to_string().contains("IOPMGetThermalWarningLevel")),
    }

    assert!(matches!(
        system_load_advisory(),
        SystemLoadAdvisoryLevel::Bad
            | SystemLoadAdvisoryLevel::Ok
            | SystemLoadAdvisoryLevel::Great
            | SystemLoadAdvisoryLevel::Unknown(_)
    ));
    if let Some(details) = copy_system_load_advisory_detailed() {
        assert_eq!(details.kind_name(), "Dictionary");
    }

    Ok(())
}
