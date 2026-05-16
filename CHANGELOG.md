# Changelog

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
