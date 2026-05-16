import CoreFoundation
import Foundation
import IOKit

@_cdecl("iokit_swift_notification_port_create")
public func iokit_swift_notification_port_create() -> UnsafeMutableRawPointer? {
    guard let port = IONotificationPortCreate(0) else {
        return nil
    }
    return retainOpaque(NotificationPortHolder(port))
}

@_cdecl("iokit_swift_notification_port_retain")
public func iokit_swift_notification_port_retain(_ port: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let port = notificationPortHolder(port) else {
        return nil
    }
    return retainOpaque(port)
}

@_cdecl("iokit_swift_notification_port_release")
public func iokit_swift_notification_port_release(_ port: UnsafeMutableRawPointer?) {
    guard let port else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(port).release()
}

@_cdecl("iokit_swift_notification_port_mach_port")
public func iokit_swift_notification_port_mach_port(_ port: UnsafeMutableRawPointer?) -> mach_port_t {
    guard let port = notificationPortHolder(port) else {
        return 0
    }
    return IONotificationPortGetMachPort(port.raw)
}

@_cdecl("iokit_swift_notification_port_run_loop_source")
public func iokit_swift_notification_port_run_loop_source(_ port: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let port = notificationPortHolder(port),
          let source = IONotificationPortGetRunLoopSource(port.raw) else {
        return nil
    }
    return UnsafeMutableRawPointer(Unmanaged.passUnretained(source as AnyObject).toOpaque())
}

@_cdecl("iokit_swift_notification_port_set_importance_receiver")
public func iokit_swift_notification_port_set_importance_receiver(_ port: UnsafeMutableRawPointer?) -> kern_return_t {
    guard let port = notificationPortHolder(port) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IONotificationPortSetImportanceReceiver(port.raw)
}
