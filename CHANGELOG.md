# Changelog

## 0.1.0

- Initial release.
- Safe wrappers for `IOServiceMatching` / `IOServiceGetMatchingService(s)` registry discovery.
- Safe `Service` / `ObjectIterator` wrappers for registry traversal and property snapshots.
- Safe `PowerAssertion` wrapper over `IOPMAssertionCreateWithName` / `IOPMAssertionRelease`.
- `CFValue` snapshots for registry properties and `IOPMCopyAssertionsByProcess`.
- Raw FFI coverage for `IORegisterForSystemPower`, `IOServiceAddMatchingNotification`, and key `IOMessage.h` constants.
