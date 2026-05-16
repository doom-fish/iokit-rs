import Foundation
import IOKit

@_cdecl("iokit_swift_connect_retain")
public func iokit_swift_connect_retain(_ connect: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let connect = connectHolder(connect) else {
        return nil
    }
    return retainOpaque(connect)
}

@_cdecl("iokit_swift_connect_release")
public func iokit_swift_connect_release(_ connect: UnsafeMutableRawPointer?) {
    guard let connect else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(connect).release()
}

@_cdecl("iokit_swift_connect_get_service")
public func iokit_swift_connect_get_service(_ connect: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let connect = connectHolder(connect) else {
        return nil
    }
    var service: io_service_t = 0
    let status = IOConnectGetService(connect.raw, &service)
    guard status == 0, service != 0 else {
        return nil
    }
    return retainOpaque(IOObjectHolder(service))
}

@_cdecl("iokit_swift_connect_set_notification_port")
public func iokit_swift_connect_set_notification_port(
    _ connect: UnsafeMutableRawPointer?,
    _ port: UnsafeMutableRawPointer?,
    _ type: UInt32,
    _ reference: UInt
) -> kern_return_t {
    guard let connect = connectHolder(connect), let port = notificationPortHolder(port) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOConnectSetNotificationPort(connect.raw, type, IONotificationPortGetMachPort(port.raw), reference)
}

@_cdecl("iokit_swift_connect_add_client")
public func iokit_swift_connect_add_client(
    _ connect: UnsafeMutableRawPointer?,
    _ client: UnsafeMutableRawPointer?
) -> kern_return_t {
    guard let connect = connectHolder(connect), let client = connectHolder(client) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOConnectAddClient(connect.raw, client.raw)
}

@_cdecl("iokit_swift_connect_call_scalar_method")
public func iokit_swift_connect_call_scalar_method(
    _ connect: UnsafeMutableRawPointer?,
    _ selector: UInt32,
    _ input: UnsafePointer<UInt64>?,
    _ inputCount: UInt32,
    _ output: UnsafeMutablePointer<UInt64>?,
    _ outputCount: UnsafeMutablePointer<UInt32>?
) -> kern_return_t {
    guard let connect = connectHolder(connect) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOConnectCallScalarMethod(connect.raw, selector, input, inputCount, output, outputCount)
}

@_cdecl("iokit_swift_connect_call_struct_method")
public func iokit_swift_connect_call_struct_method(
    _ connect: UnsafeMutableRawPointer?,
    _ selector: UInt32,
    _ input: UnsafeRawPointer?,
    _ inputLen: Int,
    _ output: UnsafeMutableRawPointer?,
    _ outputLen: UnsafeMutablePointer<Int>?
) -> kern_return_t {
    guard let connect = connectHolder(connect) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOConnectCallStructMethod(connect.raw, selector, input, inputLen, output, outputLen)
}

@_cdecl("iokit_swift_connect_call_method")
public func iokit_swift_connect_call_method(
    _ connect: UnsafeMutableRawPointer?,
    _ selector: UInt32,
    _ inputScalars: UnsafePointer<UInt64>?,
    _ inputScalarCount: UInt32,
    _ inputStruct: UnsafeRawPointer?,
    _ inputStructLen: Int,
    _ outputScalars: UnsafeMutablePointer<UInt64>?,
    _ outputScalarCount: UnsafeMutablePointer<UInt32>?,
    _ outputStruct: UnsafeMutableRawPointer?,
    _ outputStructLen: UnsafeMutablePointer<Int>?
) -> kern_return_t {
    guard let connect = connectHolder(connect) else {
        return kern_return_t(bitPattern: UInt32.max)
    }
    return IOConnectCallMethod(
        connect.raw,
        selector,
        inputScalars,
        inputScalarCount,
        inputStruct,
        inputStructLen,
        outputScalars,
        outputScalarCount,
        outputStruct,
        outputStructLen
    )
}
