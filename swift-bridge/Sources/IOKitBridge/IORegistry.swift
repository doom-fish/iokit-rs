import CoreFoundation
import Foundation
import IOKit

@_cdecl("iokit_swift_registry_entry_from_path")
public func iokit_swift_registry_entry_from_path(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else {
        return nil
    }
    let raw = IORegistryEntryFromPath(0, path)
    guard raw != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(raw))
}

@_cdecl("iokit_swift_registry_entry_retain")
public func iokit_swift_registry_entry_retain(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry) else {
        return nil
    }
    return retainOpaque(entry)
}

@_cdecl("iokit_swift_registry_entry_release")
public func iokit_swift_registry_entry_release(_ entry: UnsafeMutableRawPointer?) {
    guard let entry else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(entry).release()
}

@_cdecl("iokit_swift_registry_entry_name")
public func iokit_swift_registry_entry_name(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let entry = ioObjectHolder(entry) else {
        return nil
    }
    var buffer = [CChar](repeating: 0, count: ioNameSize)
    let status = IORegistryEntryGetName(entry.raw, &buffer)
    guard status == 0 else {
        return nil
    }
    return strdup(buffer)
}

@_cdecl("iokit_swift_registry_entry_name_in_plane")
public func iokit_swift_registry_entry_name_in_plane(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var buffer = [CChar](repeating: 0, count: ioNameSize)
    let status = IORegistryEntryGetNameInPlane(entry.raw, plane, &buffer)
    guard status == 0 else {
        return nil
    }
    return strdup(buffer)
}

@_cdecl("iokit_swift_registry_entry_location_in_plane")
public func iokit_swift_registry_entry_location_in_plane(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var buffer = [CChar](repeating: 0, count: ioNameSize)
    let status = IORegistryEntryGetLocationInPlane(entry.raw, plane, &buffer)
    guard status == 0 else {
        return nil
    }
    return strdup(buffer)
}

@_cdecl("iokit_swift_registry_entry_path")
public func iokit_swift_registry_entry_path(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var buffer = [CChar](repeating: 0, count: 512)
    let status = IORegistryEntryGetPath(entry.raw, plane, &buffer)
    guard status == 0 else {
        return nil
    }
    return strdup(buffer)
}

@_cdecl("iokit_swift_registry_entry_registry_entry_id")
public func iokit_swift_registry_entry_registry_entry_id(
    _ entry: UnsafeMutableRawPointer?,
    _ out: UnsafeMutablePointer<UInt64>?
) -> kern_return_t {
    guard let entry = ioObjectHolder(entry), let out else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IORegistryEntryGetRegistryEntryID(entry.raw, out)
}

@_cdecl("iokit_swift_registry_entry_properties")
public func iokit_swift_registry_entry_properties(_ entry: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry) else {
        return nil
    }
    var properties: Unmanaged<CFMutableDictionary>? = nil
    let status = IORegistryEntryCreateCFProperties(entry.raw, &properties, kCFAllocatorDefault, 0)
    guard status == 0 else {
        return nil
    }
    return retainedOpaque(properties)
}

@_cdecl("iokit_swift_registry_entry_property")
public func iokit_swift_registry_entry_property(
    _ entry: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let key = cfString(from: key) else {
        return nil
    }
    return retainedOpaque(IORegistryEntryCreateCFProperty(entry.raw, key, kCFAllocatorDefault, 0))
}

@_cdecl("iokit_swift_registry_entry_search_property")
public func iokit_swift_registry_entry_search_property(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?,
    _ key: UnsafePointer<CChar>?,
    _ options: IOOptionBits
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let plane, let key = cfString(from: key) else {
        return nil
    }
    guard let value = IORegistryEntrySearchCFProperty(entry.raw, plane, key, kCFAllocatorDefault, options) else {
        return nil
    }
    return retainOpaque(value)
}

@_cdecl("iokit_swift_registry_entry_parent")
public func iokit_swift_registry_entry_parent(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var parent: io_registry_entry_t = 0
    let status = IORegistryEntryGetParentEntry(entry.raw, plane, &parent)
    guard status == 0, parent != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(parent))
}

@_cdecl("iokit_swift_registry_entry_child")
public func iokit_swift_registry_entry_child(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var child: io_registry_entry_t = 0
    let status = IORegistryEntryGetChildEntry(entry.raw, plane, &child)
    guard status == 0, child != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(child))
}

@_cdecl("iokit_swift_registry_entry_parent_iterator")
public func iokit_swift_registry_entry_parent_iterator(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var iterator: io_iterator_t = 0
    let status = IORegistryEntryGetParentIterator(entry.raw, plane, &iterator)
    guard status == 0, iterator != 0 else {
        return nil
    }
    return retainOpaque(IteratorHolder(iterator))
}

@_cdecl("iokit_swift_registry_entry_child_iterator")
public func iokit_swift_registry_entry_child_iterator(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var iterator: io_iterator_t = 0
    let status = IORegistryEntryGetChildIterator(entry.raw, plane, &iterator)
    guard status == 0, iterator != 0 else {
        return nil
    }
    return retainOpaque(IteratorHolder(iterator))
}

@_cdecl("iokit_swift_registry_entry_create_iterator")
public func iokit_swift_registry_entry_create_iterator(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?,
    _ options: IOOptionBits
) -> UnsafeMutableRawPointer? {
    guard let entry = ioObjectHolder(entry), let plane else {
        return nil
    }
    var iterator: io_iterator_t = 0
    let status = IORegistryEntryCreateIterator(entry.raw, plane, options, &iterator)
    guard status == 0, iterator != 0 else {
        return nil
    }
    return retainOpaque(IteratorHolder(iterator))
}

@_cdecl("iokit_swift_registry_entry_in_plane")
public func iokit_swift_registry_entry_in_plane(
    _ entry: UnsafeMutableRawPointer?,
    _ plane: UnsafePointer<CChar>?
) -> Bool {
    guard let entry = ioObjectHolder(entry), let plane else {
        return false
    }
    return IORegistryEntryInPlane(entry.raw, plane) != 0
}
