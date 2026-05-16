# Changelog

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
