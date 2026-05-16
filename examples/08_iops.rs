use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let info = PowerSourcesInfo::copy()?;
    let source_count = info.len();
    let provider = info.providing_power_source_type();
    let warning = battery_warning_level();
    let remaining = time_remaining_estimate();

    println!("power sources = {source_count}");
    println!("provider type = {provider:?}");
    println!("battery warning = {warning:?}");
    println!("time remaining estimate = {remaining}");
    Ok(())
}
