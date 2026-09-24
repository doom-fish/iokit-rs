# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - Unreleased

### Fixed

- Use-after-free in `ServiceInterestStream` and `ServiceMatchStream`: their notification ports delivered on the concurrent global queue while unsubscribe drained only the bridge's private queue, so a callback already running could use the released bridge and the port destroyed by its `deinit`. Each port now delivers on the bridge's own serial queue; unsubscribe clears the bridge, releases the notifiers and destroys the port on that queue, drains it with `queue.sync {}`, and only then releases the bridge.
- All four streams pass a doom-fish-utils `CallbackContext` to the bridge, which retains it at registration and releases it in `deinit`; dropping a stream deactivates the context first, so no callback can reach a freed sender. Teardown reached from the delivery queue itself defers instead of deadlocking.
- `ServiceMatchStream` registered its notifications and drained the initial iterators on the caller's thread while callbacks could already run on another.
- A stream capacity of 0 panicked; it is now an `InvalidArgument` error.
- `NotificationPort::run_loop_source_raw` returned the address of a freed temporary Swift box instead of the port's `CFRunLoopSourceRef`.
- `main_port` and `main_port_from_bootstrap` linked `IOMainPort` (macOS 12) strongly, so binaries using them did not load on macOS 10.15 and 11; the symbol is now weakly linked with an `IOMasterPort` fallback.
- Floating-point `CFNumber`s were read as `Integer` (2.0) or `Unknown` (1.5), and unsigned values above `i64::MAX` came back negative.
- `Connect::call_scalar_method` and `call_method` sent a scalar count of 0 when the input slice length overflowed `u32`; they return `InvalidArgument`.
- The raw `CFNumberGetValue` declaration took the number type as `i32`; the SDK's `CFNumberType` is a `CFIndex`. A test now pins it and the raw `IOServiceAddInterestNotification` signature (6 parameters, as in the SDK).
- The README described 0.3.1 and claimed every stream drained callbacks before freeing its state; the coverage audits now state that they measure a self-selected sample in which 68 VERIFIED rows are raw FFI only and 43 more have only `unsafe` wrappers.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment directory (`usr/lib/swift-5.5/macosx`) to the link search path or the rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's `libswift_Concurrency.tbd` in every binary that depends on this crate, so linking failed next to a Swift bridge that uses newer concurrency APIs, such as apple-localauthentication's.

### Changed

- **Breaking:** `ObjectIterator` no longer implements `Clone`; a clone shared the kernel cursor with the original. Use `reset()` to iterate again.
- **Breaking:** `CFValue` reads floating-point numbers as `CFValue::Float` and unsigned values above `i64::MAX` as `CFValue::UnsignedInteger`.
- **Breaking:** raw `iokit::ffi::CFNumberGetValue` takes a `CFIndex` number type, and `kCFNumberSInt64Type` is a `CFIndex`.
- **Breaking:** zero-capacity stream subscriptions return `InvalidArgument`, and `ServiceMatchStream::subscribe` no longer rejects class names containing NUL (they match nothing).
- **Breaking:** `Connect::set_notification_port` is `unsafe`. The driver's messages to the port carry callouts (a function pointer and its argument, built from the reference the caller passes) that the port calls once it is scheduled, and ports can now be scheduled from safe code.
- **Breaking:** `HidManager::activate` and `cancel`, and `HidDevice::activate` and `cancel`, are `unsafe`. IOKit traps unless a dispatch queue was set first, which only the unsafe `set_dispatch_queue_raw` does, and activation starts the callbacks registered through the raw functions.
- **Breaking:** `NotificationPort` no longer implements `Clone`, because clones would share its scheduling state. It is `Send + Sync`; share it with `Arc`.
- `ServiceInterestEvent` moved to `io_notification_port` and `ExistingServices` to `io_service`; both are exported at the crate root and still re-exported from `async_api`.
- `Service` and `RegistryEntry` are `Send + Sync`, so `ServiceMatchEvent` is `Send + Sync` without its former `unsafe impl` over a non-`Send` `Service`.
- `rust-version` is 1.82; `apple-cf` is required at `>=0.11, <0.12` (with its `dispatch` feature) and `doom-fish-utils` at `>=0.4.1, <0.5`, which is no longer optional: the `async` feature only enables `async_api`.

### Added

- Safe notification-port scheduling: `NotificationPort::schedule_on(&DispatchQueue)` schedules a port once, on a private serial queue that targets the given queue (`IONotificationPortSetDispatchQueue`). `DispatchQueue` and `DispatchQoS` are re-exported from apple-cf.
- `NotificationPort::add_matching_notification` (with `ExistingServices`) and `add_interest_notification` return a `PortNotification` that owns its callback in a doom-fish-utils `CallbackContext`. Dropping it releases the notifier on the port's delivery queue and only then frees the callback, and IOKit delivers nothing for a released notifier; registrations keep their port alive, and the port is destroyed on its delivery queue. Callback panics are contained.
- `MatchingDictionary`: the class, name, BSD-name and registry-entry-ID dictionaries IOKit's helpers build, arbitrary matching keys, `IOPropertyMatch` entries, and USB (`usb_device`) and HID (`hid_device`) vendor/product matching, with `first_service`, `services_iterator` and `services`. The key names are exported as `PROVIDER_CLASS_KEY`, `NAME_MATCH_KEY`, `PROPERTY_MATCH_KEY`, `BSD_NAME_KEY` and `REGISTRY_ENTRY_ID_KEY`.
- `Service::set_property` and `RegistryEntry::set_property` (`IORegistryEntrySetCFProperty`).
- `ServiceMatchStream::subscribe_matching` with `ExistingServices::Deliver`, which delivers the services that already match as `Matched` events.
- `CFValue::Float`, `CFValue::UnsignedInteger`, and `From` conversions into `CFValue` for strings, booleans, integers, floats and byte vectors.

### Removed

- `impl Clone for ObjectIterator` and the Swift export it used.
- `impl Clone for NotificationPort`.
- The empty `include/IOKitBridge.h` bridge header (it declared nothing).

## [0.5.3] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.5.2] - 2026-05-18

- Completed a doc-pass over the remaining non-generated source, including helper modules, handle impls, and re-export module headers.
- Raised nightly `rustdoc` coverage for the non-generated `src/` surface to 100.0% (`cargo +nightly rustdoc --lib --all-features -- -Z unstable-options --show-coverage`).

## [0.5.1] - 2026-05-18

- Added `///` docs for the public `cf`, `error`, `io_cf`, `io_connect`, and
  `io_hi_backing_store` surfaces plus top-level module docs, raising rustdoc
  coverage above 60%.
- Kept the public docs aligned with the corresponding `IOKit` framework entry
  points by referencing the wrapped APIs directly.

## [0.5.0] - 2026-05-18

- Re-exported `CFIndex`, `CFTypeID`, `CFTimeInterval`, and `CFOptionFlags` from
  `apple_cf::raw` instead of maintaining local raw typedef duplicates in
  `src/ffi/mod.rs`.
- Widened the `apple-cf` dependency range from `>=0.8, <0.9` to
  `>=0.9, <0.10`.
- This is a breaking raw-FFI change: the public `ffi` module now sources four
  additional Core Foundation aliases from `apple-cf`.

## [0.4.0] - 2026-05-18

- Added `apple-cf` as a dependency and re-exported the raw `CF*Ref` aliases from
  `apple_cf::raw` instead of maintaining local duplicates in `src/ffi/mod.rs`.
- This is a breaking raw-FFI change: the public `ffi` module now exposes
  `apple-cf`'s nominal Core Foundation pointer aliases.

## 0.3.1

- **Panic safety**: all four `extern "C"` stream callbacks
  (`service_interest_cb`, `service_match_cb`, `power_source_cb`,
  `system_power_cb`) now wrap their body in
  `doom_fish_utils::panic_safe::catch_user_panic`, preventing undefined
  behaviour if Rust panics across the FFI boundary.
- **`SystemPowerStream` docs**: clarified that sleep/shutdown
  acknowledgments are issued **synchronously inside the IOKit callback**,
  before the event reaches Rust.  The stream is observation-only;
  added guidance on using `PowerAssertion` to delay sleep instead.
- **SAFETY comments**: added `// SAFETY:` annotations to every
  `Box::from_raw` call in the four `*Handle::drop` implementations.
- **`doom-fish-utils` version range**: widened from `"0.1"` (≡ `<0.2`)
  to `">=0.1, <0.3"` to allow the next minor release.
- **README doctest**: fixed `Some(())` → `Some(_)` in the async
  `watch_battery` example (`PowerSourceEvent` is a unit struct, not `()`).

## 0.3.0

- Added `async` feature gate with optional `doom-fish-utils` dependency.
- Added `async_api` module with four `BoundedAsyncStream`-based event streams:
  - `ServiceInterestStream` — wraps `IOServiceAddInterestNotification`.
  - `ServiceMatchStream` — wraps `IOServiceAddMatchingNotification` (matched + terminated).
  - `PowerSourceStream` — wraps `IOPSNotificationCreateRunLoopSource`.
  - `SystemPowerStream` — wraps `IORegisterForSystemPower`; auto-acknowledges sleep/shutdown messages.
- All four streams are `Drop`-safe: unsubscribing is synchronous and guaranteed to drain any in-flight callbacks before freeing the sender pointer.

## 0.2.1

- Added core `IOKitLib` helpers for `IOMainPort`, global busy/quiet queries, root-registry iteration, BSD-name matching, and `IOCatalogue*` access.
- Added `IOHIDManager` / `IOHIDDevice` wrappers plus new `io_kit` and `io_hid` examples and integration tests.
- Added `IOPMLib` assertion-metadata constants and system-load advisory wrappers.
- Added raw-ffi coverage for `IOCFPlugIn`, `IOCFSerialize` / `IOCFUnserialize`, `IODataQueue`, and `IOUserServer` gaps, and closed the remaining audit gaps for the sampled public SDK surface.

## 0.2.0

- Added a Swift bridge package and area-based Rust modules for `IOService`, `IOConnect`, `IORegistry`, `IONotificationPort`, `IOIterator`, `IOPMLib`, `IOPS`, and `IOMessage`.
- Expanded the safe API surface with typed wrappers for registry entry lookup, notification ports, user-client connections, power-source snapshots, and additional power-management queries.
- Added area-specific examples and integration tests for every requested logical area.
- Added `COVERAGE.md` documenting the requested public SDK coverage and the intentional `IOHIBackingStore` unavailability stub.
- Enabled the audited `raw-ffi` surface behind the default `raw-ffi` feature.

## 0.1.0

- Initial release.
- Safe wrappers for `IOServiceMatching` / `IOServiceGetMatchingService(s)` registry discovery.
- Safe `Service` / `ObjectIterator` wrappers for registry traversal and property snapshots.
- Safe `PowerAssertion` wrapper over `IOPMAssertionCreateWithName` / `IOPMAssertionRelease`.
- `CFValue` snapshots for registry properties and `IOPMCopyAssertionsByProcess`.
- Raw FFI coverage for `IORegisterForSystemPower`, `IOServiceAddMatchingNotification`, and key `IOMessage.h` constants.
