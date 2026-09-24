use iokit::prelude::*;
use iokit::{DispatchQoS, ExistingServices, MATCHED_NOTIFICATION};

fn main() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    let mach_port = port.mach_port();
    let run_loop_source_raw = port.run_loop_source_raw();
    println!("notification mach port = {mach_port}");
    println!("run loop source raw = 0x{run_loop_source_raw:x}");

    let queue = DispatchQueue::new("fish.doom.iokit.example", DispatchQoS::Default);
    port.schedule_on(&queue)?;
    let registration = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &MatchingDictionary::class("IOResources"),
        ExistingServices::Deliver,
        |service| println!("matched {:?}", service.class_name()),
    )?;
    drop(registration);
    Ok(())
}
