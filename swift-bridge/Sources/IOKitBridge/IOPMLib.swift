import CoreFoundation
import Foundation
import IOKit
import IOKit.pwr_mgt

@_cdecl("iokit_swift_power_find_power_management")
public func iokit_swift_power_find_power_management() -> UnsafeMutableRawPointer? {
    let raw = IOPMFindPowerManagement(0)
    guard raw != 0 else {
        return nil
    }
    return retainOpaque(ConnectHolder(raw))
}

@_cdecl("iokit_swift_power_sleep_enabled")
public func iokit_swift_power_sleep_enabled() -> Bool {
    IOPMSleepEnabled() != 0
}

@_cdecl("iokit_swift_power_get_aggressiveness")
public func iokit_swift_power_get_aggressiveness(
    _ connect: UnsafeMutableRawPointer?,
    _ type: UInt64,
    _ valueOut: UnsafeMutablePointer<UInt64>?
) -> IOReturn {
    guard let connect = connectHolder(connect), let valueOut else {
        return IOReturn(bitPattern: UInt32.max)
    }
    var value: UInt = 0
    let status = IOPMGetAggressiveness(connect.raw, UInt(type), &value)
    valueOut.pointee = UInt64(value)
    return status
}

@_cdecl("iokit_swift_power_set_aggressiveness")
public func iokit_swift_power_set_aggressiveness(
    _ connect: UnsafeMutableRawPointer?,
    _ type: UInt64,
    _ value: UInt64
) -> IOReturn {
    guard let connect = connectHolder(connect) else {
        return IOReturn(bitPattern: UInt32.max)
    }
    return IOPMSetAggressiveness(connect.raw, UInt(type), UInt(value))
}

@_cdecl("iokit_swift_power_get_thermal_warning_level")
public func iokit_swift_power_get_thermal_warning_level(_ levelOut: UnsafeMutablePointer<UInt32>?) -> IOReturn {
    guard let levelOut else {
        return IOReturn(bitPattern: UInt32.max)
    }
    return IOPMGetThermalWarningLevel(levelOut)
}

@_cdecl("iokit_swift_power_copy_assertions_by_process")
public func iokit_swift_power_copy_assertions_by_process() -> UnsafeMutableRawPointer? {
    var assertions: Unmanaged<CFDictionary>? = nil
    let status = IOPMCopyAssertionsByProcess(&assertions)
    guard status == 0 else {
        return nil
    }
    return retainedOpaque(assertions)
}

@_cdecl("iokit_swift_power_copy_assertions_status")
public func iokit_swift_power_copy_assertions_status() -> UnsafeMutableRawPointer? {
    var assertions: Unmanaged<CFDictionary>? = nil
    let status = IOPMCopyAssertionsStatus(&assertions)
    guard status == 0 else {
        return nil
    }
    return retainedOpaque(assertions)
}

@_cdecl("iokit_swift_power_copy_battery_info")
public func iokit_swift_power_copy_battery_info() -> UnsafeMutableRawPointer? {
    var info: Unmanaged<CFArray>? = nil
    let status = IOPMCopyBatteryInfo(0, &info)
    guard status == 0 else {
        return nil
    }
    return retainedOpaque(info)
}

@_cdecl("iokit_swift_power_copy_cpu_power_status")
public func iokit_swift_power_copy_cpu_power_status() -> UnsafeMutableRawPointer? {
    var statusInfo: Unmanaged<CFDictionary>? = nil
    let status = IOPMCopyCPUPowerStatus(&statusInfo)
    guard status == 0 else {
        return nil
    }
    return retainedOpaque(statusInfo)
}

@_cdecl("iokit_swift_power_copy_scheduled_power_events")
public func iokit_swift_power_copy_scheduled_power_events() -> UnsafeMutableRawPointer? {
    return retainedOpaque(IOPMCopyScheduledPowerEvents())
}

@_cdecl("iokit_swift_power_assertion_create")
public func iokit_swift_power_assertion_create(
    _ assertionType: UnsafePointer<CChar>?,
    _ level: IOPMAssertionLevel,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let assertionType = cfString(from: assertionType), let name = cfString(from: name) else {
        return nil
    }
    var assertionID: IOPMAssertionID = 0
    let status = IOPMAssertionCreateWithName(assertionType, level, name, &assertionID)
    guard status == 0 else {
        return nil
    }
    return retainOpaque(PowerAssertionHolder(assertionID))
}

@_cdecl("iokit_swift_power_assertion_declare_user_activity")
public func iokit_swift_power_assertion_declare_user_activity(
    _ name: UnsafePointer<CChar>?,
    _ userType: UInt32
) -> UnsafeMutableRawPointer? {
    guard let name = cfString(from: name) else {
        return nil
    }
    var assertionID: IOPMAssertionID = 0
    let activeType = IOPMUserActiveType(rawValue: userType) ?? IOPMUserActiveType(rawValue: 0)
    let status = IOPMAssertionDeclareUserActivity(name, activeType, &assertionID)
    guard status == 0 else {
        return nil
    }
    return retainOpaque(PowerAssertionHolder(assertionID))
}

@_cdecl("iokit_swift_power_assertion_declare_network_client_activity")
public func iokit_swift_power_assertion_declare_network_client_activity(
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let name = cfString(from: name) else {
        return nil
    }
    var assertionID: IOPMAssertionID = 0
    let status = IOPMDeclareNetworkClientActivity(name, &assertionID)
    guard status == 0 else {
        return nil
    }
    return retainOpaque(PowerAssertionHolder(assertionID))
}

@_cdecl("iokit_swift_power_assertion_retain")
public func iokit_swift_power_assertion_retain(_ assertion: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let assertion = powerAssertionHolder(assertion) else {
        return nil
    }
    return retainOpaque(assertion)
}

@_cdecl("iokit_swift_power_assertion_release")
public func iokit_swift_power_assertion_release(_ assertion: UnsafeMutableRawPointer?) {
    guard let assertion else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(assertion).release()
}

@_cdecl("iokit_swift_power_assertion_id")
public func iokit_swift_power_assertion_id(_ assertion: UnsafeMutableRawPointer?) -> IOPMAssertionID {
    guard let assertion = powerAssertionHolder(assertion) else {
        return 0
    }
    return assertion.id
}

@_cdecl("iokit_swift_power_assertion_copy_properties")
public func iokit_swift_power_assertion_copy_properties(_ assertion: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let assertion = powerAssertionHolder(assertion) else {
        return nil
    }
    return retainedOpaque(IOPMAssertionCopyProperties(assertion.id))
}
