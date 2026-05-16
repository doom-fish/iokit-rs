import Foundation
import IOKit

@_cdecl("iokit_swift_iterator_retain")
public func iokit_swift_iterator_retain(_ iterator: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let iterator = iteratorHolder(iterator) else {
        return nil
    }
    return retainOpaque(iterator)
}

@_cdecl("iokit_swift_iterator_release")
public func iokit_swift_iterator_release(_ iterator: UnsafeMutableRawPointer?) {
    guard let iterator else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(iterator).release()
}

@_cdecl("iokit_swift_iterator_is_valid")
public func iokit_swift_iterator_is_valid(_ iterator: UnsafeMutableRawPointer?) -> Bool {
    guard let iterator = iteratorHolder(iterator) else {
        return false
    }
    return IOIteratorIsValid(iterator.raw) != 0
}

@_cdecl("iokit_swift_iterator_reset")
public func iokit_swift_iterator_reset(_ iterator: UnsafeMutableRawPointer?) {
    guard let iterator = iteratorHolder(iterator) else {
        return
    }
    IOIteratorReset(iterator.raw)
}

@_cdecl("iokit_swift_iterator_next_service")
public func iokit_swift_iterator_next_service(_ iterator: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let iterator = iteratorHolder(iterator) else {
        return nil
    }
    let next = IOIteratorNext(iterator.raw)
    guard next != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(next))
}

@_cdecl("iokit_swift_iterator_next_registry_entry")
public func iokit_swift_iterator_next_registry_entry(_ iterator: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let iterator = iteratorHolder(iterator) else {
        return nil
    }
    let next = IOIteratorNext(iterator.raw)
    guard next != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(next))
}
