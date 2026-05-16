use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let assertions = copy_assertions_by_process()?;
    let assertions_kind = assertions.kind_name();
    let thermal = thermal_warning_level();
    let sleep = sleep_enabled();
    let connect = find_power_management()?;
    let has_service = connect.get_service().is_some();

    println!("assertions kind = {assertions_kind}");
    println!("sleep enabled = {sleep}");
    match thermal {
        Ok(level) => println!("thermal warning = {level:?}"),
        Err(error) => println!("thermal warning unavailable: {error}"),
    }
    println!("power management has service = {has_service}");
    Ok(())
}
