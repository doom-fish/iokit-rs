import CoreFoundation
import Dispatch
import Foundation
import IOKit
import IOKit.ps
import IOKit.pwr_mgt
import Darwin

@inline(__always)
func retainOpaque<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(Unmanaged.passRetained(object).toOpaque())
}

@inline(__always)
func retainedOpaque<T: AnyObject>(_ unmanaged: Unmanaged<T>?) -> UnsafeMutableRawPointer? {
    guard let unmanaged else {
        return nil
    }
    let object = unmanaged.takeRetainedValue()
    return retainOpaque(object)
}

@inline(__always)
func borrowedOpaque<T: AnyObject>(_ object: T?) -> UnsafeMutableRawPointer? {
    guard let object else {
        return nil
    }
    return retainOpaque(object)
}

@inline(__always)
func anyObject<T: AnyObject>(_ raw: UnsafeMutableRawPointer?, as type: T.Type) -> T? {
    guard let raw else {
        return nil
    }
    return Unmanaged<T>.fromOpaque(raw).takeUnretainedValue()
}

@inline(__always)
func dupCString(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else {
        return nil
    }
    return strdup(value)
}

@inline(__always)
func cfString(from cString: UnsafePointer<CChar>?) -> CFString? {
    guard let cString else {
        return nil
    }
    return String(cString: cString) as CFString
}

let ioNameSize = 128

@inline(__always)
func ioNameCString(from cString: UnsafePointer<CChar>?) -> UnsafePointer<CChar>? {
    cString
}

@_cdecl("iokit_swift_free_string")
public func iokit_swift_free_string(_ string: UnsafeMutablePointer<CChar>?) {
    guard let string else {
        return
    }
    free(string)
}

final class IOObjectHolder {
    let raw: io_object_t

    init(_ raw: io_object_t) {
        self.raw = raw
    }

    deinit {
        if raw != 0 {
            _ = IOObjectRelease(raw)
        }
    }
}

final class IteratorHolder {
    let raw: io_iterator_t

    init(_ raw: io_iterator_t) {
        self.raw = raw
    }

    deinit {
        if raw != 0 {
            _ = IOObjectRelease(raw)
        }
    }
}

final class ConnectHolder {
    let raw: io_connect_t

    init(_ raw: io_connect_t) {
        self.raw = raw
    }

    deinit {
        if raw != 0 {
            _ = IOServiceClose(raw)
        }
    }
}

final class NotificationPortHolder {
    let raw: IONotificationPortRef

    init(_ raw: IONotificationPortRef) {
        self.raw = raw
    }

    deinit {
        IONotificationPortDestroy(raw)
    }
}

final class PowerAssertionHolder {
    let id: IOPMAssertionID

    init(_ id: IOPMAssertionID) {
        self.id = id
    }

    deinit {
        _ = IOPMAssertionRelease(id)
    }
}

final class PowerSourcesInfoHolder {
    let info: AnyObject
    let list: [AnyObject]

    init?() {
        guard let info = IOPSCopyPowerSourcesInfo()?.takeRetainedValue() else {
            return nil
        }
        self.info = info as AnyObject
        if let list = IOPSCopyPowerSourcesList(info)?.takeRetainedValue() as? [AnyObject] {
            self.list = list
        } else {
            self.list = []
        }
    }
}

@inline(__always)
func ioObjectHolder(_ raw: UnsafeMutableRawPointer?) -> IOObjectHolder? {
    anyObject(raw, as: IOObjectHolder.self)
}

@inline(__always)
func iteratorHolder(_ raw: UnsafeMutableRawPointer?) -> IteratorHolder? {
    anyObject(raw, as: IteratorHolder.self)
}

@inline(__always)
func connectHolder(_ raw: UnsafeMutableRawPointer?) -> ConnectHolder? {
    anyObject(raw, as: ConnectHolder.self)
}

@inline(__always)
func notificationPortHolder(_ raw: UnsafeMutableRawPointer?) -> NotificationPortHolder? {
    anyObject(raw, as: NotificationPortHolder.self)
}

@inline(__always)
func powerAssertionHolder(_ raw: UnsafeMutableRawPointer?) -> PowerAssertionHolder? {
    anyObject(raw, as: PowerAssertionHolder.self)
}

@inline(__always)
func powerSourcesInfoHolder(_ raw: UnsafeMutableRawPointer?) -> PowerSourcesInfoHolder? {
    anyObject(raw, as: PowerSourcesInfoHolder.self)
}

@_cdecl("iokit_swift_wrap_service")
public func iokit_swift_wrap_service(_ service: io_service_t) -> UnsafeMutableRawPointer? {
    guard service != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(service))
}

@_cdecl("iokit_swift_service_raw")
public func iokit_swift_service_raw(_ service: UnsafeMutableRawPointer?) -> io_service_t {
    guard let service = ioObjectHolder(service) else {
        return 0
    }
    return service.raw
}

@_cdecl("iokit_swift_wrap_registry_entry")
public func iokit_swift_wrap_registry_entry(_ entry: io_registry_entry_t) -> UnsafeMutableRawPointer? {
    guard entry != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(entry))
}

@_cdecl("iokit_swift_wrap_iterator")
public func iokit_swift_wrap_iterator(_ iterator: io_iterator_t) -> UnsafeMutableRawPointer? {
    guard iterator != 0 else {
        return nil
    }
    return retainOpaque(IteratorHolder(iterator))
}

@_cdecl("iokit_swift_iterator_enter_entry")
public func iokit_swift_iterator_enter_entry(_ iterator: UnsafeMutableRawPointer?) -> kern_return_t {
    guard let iterator = iteratorHolder(iterator) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IORegistryIteratorEnterEntry(iterator.raw)
}

@_cdecl("iokit_swift_iterator_exit_entry")
public func iokit_swift_iterator_exit_entry(_ iterator: UnsafeMutableRawPointer?) -> kern_return_t {
    guard let iterator = iteratorHolder(iterator) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IORegistryIteratorExitEntry(iterator.raw)
}
