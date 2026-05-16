use iokit::prelude::*;

fn main() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    let mach_port = port.mach_port();
    let run_loop_source_raw = port.run_loop_source_raw();
    println!("notification mach port = {mach_port}");
    println!("run loop source raw = 0x{run_loop_source_raw:x}");
    Ok(())
}
