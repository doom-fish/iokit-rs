# iokit

Safe Rust bindings for Apple's [IOKit](https://developer.apple.com/documentation/iokit) user-space APIs on macOS via a Swift bridge.

> **Status:** v0.3.0 adds a Tier-2 `async` feature with four `BoundedAsyncStream`-based event streams (`ServiceInterestStream`, `ServiceMatchStream`, `PowerSourceStream`, `SystemPowerStream`). v0.2.1 adds audited `IOKitLib`/`IOKitServer` helpers for main-port lookup, global busy/quiet queries, root-registry traversal, BSD-name matching, and `IOCatalogue*`, plus `IOHIDManager` / `IOHIDDevice` wrappers and expanded `IOPMLib` load-advisory coverage. `IOCFPlugIn`, `IOCFSerialize` / `IOCFUnserialize`, `IODataQueue`, and `IOUserServer` are now available through `raw-ffi`. `IOHIBackingStore` remains intentionally unavailable because it is private SDK surface and is not present in the public macOS headers.

## Quick start

```rust,no_run
use iokit::prelude::*;

fn main() -> Result<()> {
    if let Some(service) = matching_service("IOResources")? {
        let class_name = service.class_name()?;
        let path = service.registry_entry()?.path(SERVICE_PLANE)?;
        println!("matched class = {class_name}");
        println!("registry path = {path}");
    }

    let assertions = copy_assertions_by_process()?;
    println!("assertions snapshot kind = {}", assertions.kind_name());
    Ok(())
}
```

## Module map

- `io_kit` — main-port lookup, global busy/quiet queries, root-registry iteration, BSD-name matching, receive-port creation, and `IOCatalogue*` helpers.
- `io_service` — service matching, class/bundle metadata, registry-style helpers, and `IOServiceOpen`.
- `io_connect` — user-client connection handles plus scalar/struct method calls.
- `io_registry` — registry path lookup, names, paths, properties, and traversal.
- `io_notification_port` — notification port lifecycle plus Mach-port/run-loop access.
- `io_iterator` — iterator reset/validation, registry enter/exit, and typed service/registry iteration.
- `io_hid` — `IOHIDManager` / `IOHIDDevice` wrappers for enumeration, properties, reports, and low-level callback registration.
- `io_pm` — `IOPMLib` snapshots, aggressiveness, thermal warning lookup, load advisory, and power assertions.
- `io_cf` — `IOCFSerialize` / `IOCFUnserialize` helpers for CoreFoundation snapshots.
- `iops` — power-source snapshots, provider type, battery warning, and time remaining.
- `io_message` — typed wrappers for the public `IOMessage.h` constants.
- `io_hi_backing_store` — compatibility stub documenting public-SDK unavailability.
- `async_api` *(requires `async` feature)* — `BoundedAsyncStream`-based event streams for service interest, service match, power-source change, and system power notifications.

## Async streams

Enable the `async` feature to get four ready-to-use event streams:

```toml
[dependencies]
iokit = { version = "0.3", features = ["async"] }
```

```rust,no_run
use iokit::async_api::PowerSourceStream;

async fn watch_battery() {
    let stream = PowerSourceStream::subscribe(16).expect("subscribe");
    while let Some(()) = stream.next().await {
        println!("power-source changed");
    }
}
```

All streams implement `Drop`-safe cancellation: dropping the stream (or its
`SubscriptionHandle`) synchronously drains any in-flight callbacks before
freeing internal resources.

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 02_io_service
cargo run --example 03_io_connect
cargo run --example 04_io_registry
cargo run --example 05_io_notification_port
cargo run --example 06_io_iterator
cargo run --example 07_iopm
cargo run --example 08_iops
cargo run --example 09_io_message
cargo run --example 10_iohi_backing_store
cargo run --example 11_io_kit
cargo run --example 12_io_hid
cargo run --features async --example 13_async_stream
```

## Raw FFI

The `raw-ffi` feature is enabled by default and re-exports the audited public headers through `iokit::ffi`.

## Coverage notes

- `COVERAGE.md` tracks the requested area-by-area public SDK coverage.
- `IOPMGetThermalWarningLevel` can legitimately return an `IOReturn` error on machines that do not expose the metric; the safe wrapper preserves that result.
- `IOHIBackingStore` remains unavailable on purpose because the public macOS SDK does not ship headers for it.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
