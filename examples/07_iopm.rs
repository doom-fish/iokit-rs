use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let assertions = copy_assertions_by_process()?;
    let assertions_kind = assertions.kind_name();
    let thermal = thermal_warning_level();
    let advisory = system_load_advisory();
    let advisory_details = copy_system_load_advisory_detailed();
    let sleep = sleep_enabled();
    let connect = find_power_management()?;
    let has_service = connect.get_service().is_some();

    println!("assertions kind = {assertions_kind}");
    println!("sleep enabled = {sleep}");
    println!("system load advisory = {advisory:?}");
    println!(
        "system load advisory details = {}",
        advisory_details
            .as_ref()
            .map_or("<none>", |details| details.kind_name())
    );
    match thermal {
        Ok(level) => println!("thermal warning = {level:?}"),
        Err(error) => println!("thermal warning unavailable: {error}"),
    }
    println!("power management has service = {has_service}");
    Ok(())
}
