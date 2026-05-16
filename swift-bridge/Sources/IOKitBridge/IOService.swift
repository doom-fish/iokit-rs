import Foundation
import IOKit

@_cdecl("iokit_swift_service_matching")
public func iokit_swift_service_matching(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else {
        return nil
    }
    let raw = IOServiceGetMatchingService(0, IOServiceMatching(name))
    guard raw != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(raw))
}

@_cdecl("iokit_swift_service_name_matching")
public func iokit_swift_service_name_matching(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else {
        return nil
    }
    let raw = IOServiceGetMatchingService(0, IOServiceNameMatching(name))
    guard raw != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(raw))
}

@_cdecl("iokit_swift_service_matching_entry_id")
public func iokit_swift_service_matching_entry_id(_ entryID: UInt64) -> UnsafeMutableRawPointer? {
    let raw = IOServiceGetMatchingService(0, IORegistryEntryIDMatching(entryID))
    guard raw != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(raw))
}

@_cdecl("iokit_swift_matching_services")
public func iokit_swift_matching_services(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else {
        return nil
    }
    var iterator: io_iterator_t = 0
    let status = IOServiceGetMatchingServices(0, IOServiceMatching(name), &iterator)
    guard status == 0, iterator != 0 else {
        return nil
    }
    return retainOpaque(IteratorHolder(iterator))
}

@_cdecl("iokit_swift_name_matching_services")
public func iokit_swift_name_matching_services(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else {
        return nil
    }
    var iterator: io_iterator_t = 0
    let status = IOServiceGetMatchingServices(0, IOServiceNameMatching(name), &iterator)
    guard status == 0, iterator != 0 else {
        return nil
    }
    return retainOpaque(IteratorHolder(iterator))
}

@_cdecl("iokit_swift_service_retain")
public func iokit_swift_service_retain(_ service: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let service = ioObjectHolder(service) else {
        return nil
    }
    return retainOpaque(service)
}

@_cdecl("iokit_swift_service_release")
public func iokit_swift_service_release(_ service: UnsafeMutableRawPointer?) {
    guard let service else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(service).release()
}

@_cdecl("iokit_swift_service_class_name")
public func iokit_swift_service_class_name(_ service: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let service = ioObjectHolder(service) else {
        return nil
    }
    var buffer = [CChar](repeating: 0, count: ioNameSize)
    let status = IOObjectGetClass(service.raw, &buffer)
    guard status == 0 else {
        return nil
    }
    return strdup(buffer)
}

@_cdecl("iokit_swift_service_bundle_identifier")
public func iokit_swift_service_bundle_identifier(_ service: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let service = ioObjectHolder(service) else {
        return nil
    }
    guard let className = IOObjectCopyClass(service.raw)?.takeRetainedValue(),
          let bundleIdentifier = IOObjectCopyBundleIdentifierForClass(className)?.takeRetainedValue() else {
        return nil
    }
    return dupCString(bundleIdentifier as String)
}

@_cdecl("iokit_swift_service_superclass_name")
public func iokit_swift_service_superclass_name(_ service: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let service = ioObjectHolder(service) else {
        return nil
    }
    guard let className = IOObjectCopyClass(service.raw)?.takeRetainedValue(),
          let superclass = IOObjectCopySuperclassForClass(className)?.takeRetainedValue() else {
        return nil
    }
    return dupCString(superclass as String)
}

@_cdecl("iokit_swift_service_conforms_to")
public func iokit_swift_service_conforms_to(
    _ service: UnsafeMutableRawPointer?,
    _ className: UnsafePointer<CChar>?
) -> Bool {
    guard let service = ioObjectHolder(service), let className else {
        return false
    }
    return IOObjectConformsTo(service.raw, className) != 0
}

@_cdecl("iokit_swift_service_is_equal_to")
public func iokit_swift_service_is_equal_to(
    _ service: UnsafeMutableRawPointer?,
    _ other: UnsafeMutableRawPointer?
) -> Bool {
    guard let service = ioObjectHolder(service), let other = ioObjectHolder(other) else {
        return false
    }
    return IOObjectIsEqualTo(service.raw, other.raw) != 0
}

@_cdecl("iokit_swift_service_kernel_retain_count")
public func iokit_swift_service_kernel_retain_count(_ service: UnsafeMutableRawPointer?) -> UInt32 {
    guard let service = ioObjectHolder(service) else {
        return 0
    }
    return IOObjectGetKernelRetainCount(service.raw)
}

@_cdecl("iokit_swift_service_user_retain_count")
public func iokit_swift_service_user_retain_count(_ service: UnsafeMutableRawPointer?) -> UInt32 {
    guard let service = ioObjectHolder(service) else {
        return 0
    }
    return IOObjectGetUserRetainCount(service.raw)
}

@_cdecl("iokit_swift_service_retain_count")
public func iokit_swift_service_retain_count(_ service: UnsafeMutableRawPointer?) -> UInt32 {
    guard let service = ioObjectHolder(service) else {
        return 0
    }
    return IOObjectGetRetainCount(service.raw)
}

@_cdecl("iokit_swift_service_busy_state")
public func iokit_swift_service_busy_state(
    _ service: UnsafeMutableRawPointer?,
    _ busyState: UnsafeMutablePointer<UInt32>?
) -> kern_return_t {
    guard let service, let busyState else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    let holder = ioObjectHolder(service)
    guard let holder else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOServiceGetBusyState(holder.raw, busyState)
}

@_cdecl("iokit_swift_service_wait_quiet")
public func iokit_swift_service_wait_quiet(
    _ service: UnsafeMutableRawPointer?,
    _ seconds: UInt32
) -> kern_return_t {
    guard let holder = ioObjectHolder(service) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    var waitTime = mach_timespec_t(tv_sec: seconds, tv_nsec: 0)
    return IOServiceWaitQuiet(holder.raw, &waitTime)
}

@_cdecl("iokit_swift_service_authorize")
public func iokit_swift_service_authorize(
    _ service: UnsafeMutableRawPointer?,
    _ options: UInt32
) -> kern_return_t {
    guard let holder = ioObjectHolder(service) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOServiceAuthorize(holder.raw, options)
}

@_cdecl("iokit_swift_service_open")
public func iokit_swift_service_open(
    _ service: UnsafeMutableRawPointer?,
    _ type: UInt32,
    _ connectOut: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> kern_return_t {
    guard let holder = ioObjectHolder(service), let connectOut else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    var connect: io_connect_t = 0
    let status = IOServiceOpen(holder.raw, mach_task_self_, type, &connect)
    guard status == 0, connect != 0 else {
        connectOut.pointee = nil
        return status
    }
    connectOut.pointee = retainOpaque(ConnectHolder(connect))
    return status
}

@_cdecl("iokit_swift_service_as_registry_entry")
public func iokit_swift_service_as_registry_entry(_ service: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let service = ioObjectHolder(service) else {
        return nil
    }
    return retainOpaque(service)
}
