# iokit

Safe Rust bindings for Apple's [IOKit](https://developer.apple.com/documentation/iokit) user-space APIs on macOS — registry lookup, property snapshots, service iteration, and power assertions.

> **Status:** experimental. v0.1 focuses on the stable C APIs that are most useful from user space: service matching, registry traversal, power assertions, assertion snapshots, and the raw notification/power-management entrypoints in `ffi`.

Pure C — **zero Swift bridge**.

## Quick start

```rust,no_run
use iokit::prelude::*;

fn main() -> Result<()> {
    if let Some(service) = matching_service("IOResources")? {
        println!("matched class = {}", service.class_name()?);
        println!("IOClass property = {:?}", service.property("IOClass")?);
    }

    let assertions = copy_assertions_by_process()?;
    println!("assertions snapshot kind = {}", assertions.kind_name());
    Ok(())
}
```

## Smoke example

```bash
cargo run --example 01_smoke
```

Expected output (values vary by machine):

```text
matched class: IOResources
io class property: Some(String("IOResources"))
child count on IOService plane: 12
assertions snapshot kind: Dictionary
✅ iokit registry + power OK
```

## Notes

- `matching_service` / `matching_services` consume an `IOServiceMatching` dictionary and wrap the returned handles with RAII `IOObjectRelease` cleanup.
- `Service::property` converts the CoreFoundation snapshot into a recursive `CFValue` enum with string, integer, boolean, data, array, and dictionary variants.
- Raw notification APIs such as `IORegisterForSystemPower` and `IOServiceAddMatchingNotification` are exposed in `ffi`; v0.1 does not yet wrap their callback lifetimes in a safe Rust abstraction.
- The smoke example is read-only. It does **not** create a sleep assertion or request system power changes.

## Roadmap

- [x] `matching_service` / `matching_services`
- [x] `Service::{class_name, property, parent, children}`
- [x] `PowerAssertion::{create, create_with_level}`
- [x] `copy_assertions_by_process`
- [x] `PowerMessage` constants for `IORegisterForSystemPower`
- [ ] Safe callback wrappers for `IORegisterForSystemPower`
- [ ] Safe callback wrappers for `IOServiceAddMatchingNotification`
- [ ] Richer typed wrappers for common registry property schemas

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
