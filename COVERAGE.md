# Coverage

This document tracks the requested logical areas for `iokit` v0.2.0.

## Requested public SDK areas

| Area | Swift bridge | Rust module | Coverage summary | Example | Test |
| --- | --- | --- | --- | --- | --- |
| `IOService` | `IOService.swift` | `io_service.rs` | Matching, class/bundle metadata, retain counts, busy state, quiet wait, authorize, open, and registry-style parent/child/property helpers. | `examples/02_io_service.rs` | `tests/io_service.rs` |
| `IOConnect` | `IOConnect.swift` | `io_connect.rs` | Connection retention, service lookup, notification-port hookup, add-client, scalar calls, struct calls, and combined method calls. | `examples/03_io_connect.rs` | `tests/io_connect.rs` |
| `IORegistry` | `IORegistry.swift` | `io_registry.rs` | Path lookup, names, locations, paths, entry IDs, property snapshots, search, parent/child lookup, iterators, and plane membership. | `examples/04_io_registry.rs` | `tests/io_registry.rs` |
| `IONotificationPort` | `IONotificationPort.swift` | `io_notification_port.rs` | Port creation/destruction, Mach-port access, run-loop source access, and importance receiver setup. | `examples/05_io_notification_port.rs` | `tests/io_notification_port.rs` |
| `IOIterator` | `IOIterator.swift` | `io_iterator.rs` | Typed iterator retain/release, validity, reset, next-service, and next-registry-entry traversal. | `examples/06_io_iterator.rs` | `tests/io_iterator.rs` |
| `IOPMLib` | `IOPMLib.swift` | `io_pm.rs` | Power-management connection lookup, sleep-enabled state, aggressiveness get/set, thermal warning lookup, assertions snapshots, battery/CPU/scheduled-event snapshots, and assertion RAII wrappers. | `examples/07_iopm.rs` | `tests/iopm.rs` |
| `IOPS` | `IOPS.swift` | `iops.rs` | Power-source info snapshots, descriptions, provider type, external adapter details, time remaining, and battery warning level. | `examples/08_iops.rs` | `tests/iops.rs` |
| `IOMessage` | `IOMessage.swift` | `io_message.rs` | Typed coverage for all 28 public `kIOMessage*` constants requested from `IOMessage.h`. | `examples/09_io_message.rs` | `tests/io_message.rs` |
| `IOHIBackingStore` | `IOHIBackingStore.swift` | `io_hi_backing_store.rs` | Public-SDK compatibility stub only. The symbol is not declared in the public macOS IOKit headers, so the crate reports unavailability instead of binding private API. | `examples/10_iohi_backing_store.rs` | `tests/iohi_backing_store.rs` |

## Notes

- The `raw-ffi` feature is enabled by default and re-exports the expanded audited C surface as `iokit::ffi`.
- `IOHIBackingStore` is deliberately documented as unavailable. The crate does not bind private Apple SDK surface.
- `IOPMGetThermalWarningLevel` may return an `IOReturn` error on hardware that does not expose the metric; the safe wrapper preserves the system result instead of guessing.
