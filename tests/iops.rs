use iokit::prelude::*;

#[test]
fn reads_power_source_state() -> iokit::Result<()> {
    let info = PowerSourcesInfo::copy()?;
    if info.is_empty() {
        assert!(info.description(0).is_none());
    } else {
        assert!(info.description(0).is_some());
    }

    assert!(time_remaining_estimate().is_finite());
    assert!(matches!(
        battery_warning_level(),
        BatteryWarningLevel::None
            | BatteryWarningLevel::Early
            | BatteryWarningLevel::Final
            | BatteryWarningLevel::Unknown(_)
    ));
    Ok(())
}
