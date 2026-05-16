use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let available = public_sdk_available();
    let reason = unavailability_reason()?;
    println!("public sdk available = {available}");
    println!("reason = {reason}");
    Ok(())
}
