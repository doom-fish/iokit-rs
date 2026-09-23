# iokit

Safe Rust bindings for Apple's [IOKit](https://developer.apple.com/documentation/iokit) user-space APIs on macOS via a Swift bridge.

Requires macOS 10.15 or later. See [`CHANGELOG.md`](CHANGELOG.md) for what changed in each release, including the breaking changes in 0.6.

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
- `io_service` — service matching (by class, name, BSD name or entry ID, or with a `MatchingDictionary` for property and USB/HID vendor-product matching), class/bundle metadata, registry-style helpers, `set_property`, and `IOServiceOpen`.
- `io_connect` — user-client connection handles plus scalar/struct method calls.
- `io_registry` — registry path lookup, names, paths, properties, and traversal.
- `io_notification_port` — notification port lifecycle, Mach-port access, and the run-loop source as a raw pointer (scheduling a port is not wrapped safely; use `async_api`).
- `io_iterator` — iterator reset/validation, registry enter/exit, and typed service/registry iteration (iterators are not `Clone`, since copies would share the kernel cursor).
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
iokit = { version = "0.6", features = ["async"] }
```

```rust,no_run
use iokit::async_api::PowerSourceStream;

async fn watch_battery() {
    let stream = PowerSourceStream::subscribe(16).expect("subscribe");
    while let Some(_) = stream.next().await {
        println!("power-source changed");
    }
}
```

Dropping a stream deactivates its callback context, stops the notification,
and drains in-flight callbacks on the bridge's private serial queue before the
bridge and its reference to the context are released. Streams, their events and
`Service` handles are `Send + Sync`, so a stream can be dropped on any thread.

`ServiceMatchStream::subscribe` skips services that already match;
`ServiceMatchStream::subscribe_matching(&matching, ExistingServices::Deliver, capacity)`
delivers them as `Matched` events first. Buffers are lossy: once `capacity`
events are queued the oldest is dropped, and a capacity of 0 is an error.

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
