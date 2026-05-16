import CoreFoundation
import Foundation
import IOKit.ps

@_cdecl("iokit_swift_iops_power_sources_info_create")
public func iokit_swift_iops_power_sources_info_create() -> UnsafeMutableRawPointer? {
    guard let info = PowerSourcesInfoHolder() else {
        return nil
    }
    return retainOpaque(info)
}

@_cdecl("iokit_swift_iops_power_sources_info_retain")
public func iokit_swift_iops_power_sources_info_retain(_ info: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let info = powerSourcesInfoHolder(info) else {
        return nil
    }
    return retainOpaque(info)
}

@_cdecl("iokit_swift_iops_power_sources_info_release")
public func iokit_swift_iops_power_sources_info_release(_ info: UnsafeMutableRawPointer?) {
    guard let info else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(info).release()
}

@_cdecl("iokit_swift_iops_power_sources_info_count")
public func iokit_swift_iops_power_sources_info_count(_ info: UnsafeMutableRawPointer?) -> Int {
    guard let info = powerSourcesInfoHolder(info) else {
        return 0
    }
    return info.list.count
}

@_cdecl("iokit_swift_iops_power_sources_info_description")
public func iokit_swift_iops_power_sources_info_description(
    _ info: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let info = powerSourcesInfoHolder(info), index >= 0, index < info.list.count else {
        return nil
    }
    let source = info.list[index]
    guard let description = IOPSGetPowerSourceDescription(info.info, source) else {
        return nil
    }
    return retainOpaque(description as AnyObject)
}

@_cdecl("iokit_swift_iops_power_sources_info_provider_type")
public func iokit_swift_iops_power_sources_info_provider_type(_ info: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let info = powerSourcesInfoHolder(info),
          let providerType = IOPSGetProvidingPowerSourceType(info.info) else {
        return nil
    }
    return dupCString(providerType.takeUnretainedValue() as String)
}

@_cdecl("iokit_swift_iops_copy_external_power_adapter_details")
public func iokit_swift_iops_copy_external_power_adapter_details() -> UnsafeMutableRawPointer? {
    return retainedOpaque(IOPSCopyExternalPowerAdapterDetails())
}

@_cdecl("iokit_swift_iops_get_time_remaining_estimate")
public func iokit_swift_iops_get_time_remaining_estimate() -> CFTimeInterval {
    IOPSGetTimeRemainingEstimate()
}

@_cdecl("iokit_swift_iops_get_battery_warning_level")
public func iokit_swift_iops_get_battery_warning_level() -> UInt32 {
    UInt32(IOPSGetBatteryWarningLevel().rawValue)
}
