# Changelog

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
