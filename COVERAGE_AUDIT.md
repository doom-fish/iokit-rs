# iokit coverage audit (vs MacOSX26.2.sdk)

> Scope note: IOKit.framework is too large for a complete one-pass diff, so this audit samples 300 public user-space symbols across `IOKitLib.h`, `IOPMLib.h`, `IOPowerSources.h`, `IOMessage.h`, `IOCFPlugIn.h`, `IOCFSerialize.h`, `IOCFUnserialize.h`, `IODataQueueClient.h`, `IOUserServer.h`, `IOHIDManager.h`, and `IOHIDDevice.h`. VERIFIED counts include both the high-level safe API and the default `raw-ffi` re-exports.

SDK_PUBLIC_SYMBOLS: 300
VERIFIED: 166
GAPS: 129
EXEMPT: 5
COVERAGE_PCT: 56.27%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `IONotificationPortRef` | type | `IOKitLib.h` | `iokit::ffi::IONotificationPortRef` |
| `IOServiceMatchingCallback` | type | `IOKitLib.h` | `iokit::ffi::IOServiceMatchingCallback` |
| `IOServiceInterestCallback` | type | `IOKitLib.h` | `iokit::ffi::IOServiceInterestCallback` |
| `IONotificationPortCreate` | function | `IOKitLib.h` | `iokit::ffi::IONotificationPortCreate`; `iokit::NotificationPort::new` |
| `IONotificationPortDestroy` | function | `IOKitLib.h` | `iokit::ffi::IONotificationPortDestroy` |
| `IONotificationPortGetRunLoopSource` | function | `IOKitLib.h` | `iokit::ffi::IONotificationPortGetRunLoopSource`; `iokit::NotificationPort::run_loop_source_raw` |
| `IONotificationPortGetMachPort` | function | `IOKitLib.h` | `iokit::ffi::IONotificationPortGetMachPort`; `iokit::NotificationPort::mach_port` |
| `IONotificationPortSetImportanceReceiver` | function | `IOKitLib.h` | `iokit::ffi::IONotificationPortSetImportanceReceiver`; `iokit::NotificationPort::set_importance_receiver` |
| `IONotificationPortSetDispatchQueue` | function | `IOKitLib.h` | `iokit::ffi::IONotificationPortSetDispatchQueue` |
| `IOObjectRelease` | function | `IOKitLib.h` | `iokit::ffi::IOObjectRelease` |
| `IOObjectRetain` | function | `IOKitLib.h` | `iokit::ffi::IOObjectRetain` |
| `IOObjectGetClass` | function | `IOKitLib.h` | `iokit::ffi::IOObjectGetClass`; `iokit::Service::class_name` |
| `IOObjectCopyClass` | function | `IOKitLib.h` | `iokit::ffi::IOObjectCopyClass`; `iokit::Service::class_name` |
| `IOObjectCopySuperclassForClass` | function | `IOKitLib.h` | `iokit::ffi::IOObjectCopySuperclassForClass`; `iokit::Service::superclass_name` |
| `IOObjectCopyBundleIdentifierForClass` | function | `IOKitLib.h` | `iokit::ffi::IOObjectCopyBundleIdentifierForClass`; `iokit::Service::bundle_identifier` |
| `IOObjectConformsTo` | function | `IOKitLib.h` | `iokit::ffi::IOObjectConformsTo`; `iokit::Service::conforms_to` |
| `IOObjectIsEqualTo` | function | `IOKitLib.h` | `iokit::ffi::IOObjectIsEqualTo`; `iokit::Service::is_equal_to` |
| `IOObjectGetKernelRetainCount` | function | `IOKitLib.h` | `iokit::ffi::IOObjectGetKernelRetainCount`; `iokit::Service::kernel_retain_count` |
| `IOObjectGetUserRetainCount` | function | `IOKitLib.h` | `iokit::ffi::IOObjectGetUserRetainCount`; `iokit::Service::user_retain_count` |
| `IOObjectGetRetainCount` | function | `IOKitLib.h` | `iokit::ffi::IOObjectGetRetainCount`; `iokit::Service::retain_count` |
| `IOIteratorNext` | function | `IOKitLib.h` | `iokit::ffi::IOIteratorNext`; `iokit::ObjectIterator::next_service` |
| `IOIteratorReset` | function | `IOKitLib.h` | `iokit::ffi::IOIteratorReset`; `iokit::ObjectIterator::reset` |
| `IOIteratorIsValid` | function | `IOKitLib.h` | `iokit::ffi::IOIteratorIsValid`; `iokit::ObjectIterator::is_valid` |
| `IOServiceGetMatchingService` | function | `IOKitLib.h` | `iokit::ffi::IOServiceGetMatchingService`; `iokit::matching_service` |
| `IOServiceGetMatchingServices` | function | `IOKitLib.h` | `iokit::ffi::IOServiceGetMatchingServices`; `iokit::matching_services_iterator`; `iokit::matching_services` |
| `IOServiceAddNotification` | function | `IOKitLib.h` | `iokit::ffi::IOServiceAddNotification` |
| `IOServiceAddMatchingNotification` | function | `IOKitLib.h` | `iokit::ffi::IOServiceAddMatchingNotification` |
| `IOServiceAddInterestNotification` | function | `IOKitLib.h` | `iokit::ffi::IOServiceAddInterestNotification` |
| `IOServiceMatchPropertyTable` | function | `IOKitLib.h` | `iokit::ffi::IOServiceMatchPropertyTable` |
| `IOServiceGetBusyState` | function | `IOKitLib.h` | `iokit::ffi::IOServiceGetBusyState`; `iokit::Service::busy_state` |
| `IOServiceWaitQuiet` | function | `IOKitLib.h` | `iokit::ffi::IOServiceWaitQuiet`; `iokit::Service::wait_quiet` |
| `IOServiceOpen` | function | `IOKitLib.h` | `iokit::ffi::IOServiceOpen`; `iokit::Service::open` |
| `IOServiceRequestProbe` | function | `IOKitLib.h` | `iokit::ffi::IOServiceRequestProbe` |
| `IOServiceAuthorize` | function | `IOKitLib.h` | `iokit::ffi::IOServiceAuthorize`; `iokit::Service::authorize` |
| `IOServiceOpenAsFileDescriptor` | function | `IOKitLib.h` | `iokit::ffi::IOServiceOpenAsFileDescriptor` |
| `IOServiceClose` | function | `IOKitLib.h` | `iokit::ffi::IOServiceClose` |
| `IOConnectAddRef` | function | `IOKitLib.h` | `iokit::ffi::IOConnectAddRef` |
| `IOConnectRelease` | function | `IOKitLib.h` | `iokit::ffi::IOConnectRelease` |
| `IOConnectGetService` | function | `IOKitLib.h` | `iokit::ffi::IOConnectGetService`; `iokit::Connect::get_service` |
| `IOConnectSetNotificationPort` | function | `IOKitLib.h` | `iokit::ffi::IOConnectSetNotificationPort`; `iokit::Connect::set_notification_port` |
| `IOConnectMapMemory` | function | `IOKitLib.h` | `iokit::ffi::IOConnectMapMemory` |
| `IOConnectMapMemory64` | function | `IOKitLib.h` | `iokit::ffi::IOConnectMapMemory64` |
| `IOConnectUnmapMemory` | function | `IOKitLib.h` | `iokit::ffi::IOConnectUnmapMemory` |
| `IOConnectUnmapMemory64` | function | `IOKitLib.h` | `iokit::ffi::IOConnectUnmapMemory64` |
| `IOConnectSetCFProperties` | function | `IOKitLib.h` | `iokit::ffi::IOConnectSetCFProperties` |
| `IOConnectSetCFProperty` | function | `IOKitLib.h` | `iokit::ffi::IOConnectSetCFProperty` |
| `IOConnectTrap0` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap0` |
| `IOConnectTrap1` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap1` |
| `IOConnectTrap2` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap2` |
| `IOConnectTrap3` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap3` |
| `IOConnectTrap4` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap4` |
| `IOConnectTrap5` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap5` |
| `IOConnectTrap6` | function | `IOKitLib.h` | `iokit::ffi::IOConnectTrap6` |
| `IOConnectAddClient` | function | `IOKitLib.h` | `iokit::ffi::IOConnectAddClient`; `iokit::Connect::add_client` |
| `IORegistryEntryFromPath` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryFromPath`; `iokit::RegistryEntry::from_path` |
| `IORegistryEntryCreateIterator` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryCreateIterator`; `iokit::RegistryEntry::create_iterator` |
| `IORegistryEntryGetName` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetName`; `iokit::RegistryEntry::name` |
| `IORegistryEntryGetNameInPlane` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetNameInPlane`; `iokit::RegistryEntry::name_in_plane` |
| `IORegistryEntryGetLocationInPlane` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetLocationInPlane`; `iokit::RegistryEntry::location_in_plane` |
| `IORegistryEntryGetPath` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetPath`; `iokit::RegistryEntry::path` |
| `IORegistryEntryGetRegistryEntryID` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetRegistryEntryID`; `iokit::RegistryEntry::registry_entry_id` |
| `IORegistryEntryCreateCFProperties` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryCreateCFProperties`; `iokit::RegistryEntry::properties` |
| `IORegistryEntryCreateCFProperty` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryCreateCFProperty`; `iokit::RegistryEntry::property` |
| `IORegistryEntrySearchCFProperty` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntrySearchCFProperty`; `iokit::RegistryEntry::search_property` |
| `IORegistryEntryGetProperty` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetProperty` |
| `IORegistryEntrySetCFProperties` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntrySetCFProperties` |
| `IORegistryEntrySetCFProperty` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntrySetCFProperty` |
| `IORegistryEntryGetChildIterator` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetChildIterator`; `iokit::RegistryEntry::child_iterator` |
| `IORegistryEntryGetChildEntry` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetChildEntry`; `iokit::RegistryEntry::child` |
| `IORegistryEntryGetParentIterator` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetParentIterator`; `iokit::RegistryEntry::parent_iterator` |
| `IORegistryEntryGetParentEntry` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryGetParentEntry`; `iokit::RegistryEntry::parent` |
| `IORegistryEntryInPlane` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryInPlane`; `iokit::RegistryEntry::in_plane` |
| `IOServiceMatching` | function | `IOKitLib.h` | `iokit::ffi::IOServiceMatching`; `iokit::matching_service`; `iokit::matching_services_iterator` |
| `IOServiceNameMatching` | function | `IOKitLib.h` | `iokit::ffi::IOServiceNameMatching`; `iokit::name_matching_service`; `iokit::name_matching_services_iterator` |
| `IORegistryEntryIDMatching` | function | `IOKitLib.h` | `iokit::ffi::IORegistryEntryIDMatching`; `iokit::matching_service_entry_id` |
| `IOServiceOFPathToBSDName` | function | `IOKitLib.h` | `iokit::ffi::IOServiceOFPathToBSDName` |
| `IOConnectMethodScalarIScalarO` | function | `IOKitLib.h` | `iokit::ffi::IOConnectMethodScalarIScalarO` |
| `IOConnectMethodScalarIStructureO` | function | `IOKitLib.h` | `iokit::ffi::IOConnectMethodScalarIStructureO` |
| `IOConnectMethodScalarIStructureI` | function | `IOKitLib.h` | `iokit::ffi::IOConnectMethodScalarIStructureI` |
| `IOConnectMethodStructureIStructureO` | function | `IOKitLib.h` | `iokit::ffi::IOConnectMethodStructureIStructureO` |
| `IOPMFindPowerManagement` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMFindPowerManagement`; `iokit::find_power_management` |
| `IOPMSetAggressiveness` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMSetAggressiveness`; `iokit::set_aggressiveness` |
| `IOPMGetAggressiveness` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMGetAggressiveness`; `iokit::get_aggressiveness` |
| `IOPMSleepEnabled` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMSleepEnabled`; `iokit::sleep_enabled` |
| `IOPMSleepSystem` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMSleepSystem` |
| `IOPMCopyBatteryInfo` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMCopyBatteryInfo`; `iokit::copy_battery_info` |
| `IORegisterForSystemPower` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IORegisterForSystemPower` |
| `IODeregisterApp` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IODeregisterApp` |
| `IODeregisterForSystemPower` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IODeregisterForSystemPower` |
| `IOAllowPowerChange` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOAllowPowerChange` |
| `IOCancelPowerChange` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOCancelPowerChange` |
| `IOPMSchedulePowerEvent` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMSchedulePowerEvent` |
| `IOPMCancelScheduledPowerEvent` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMCancelScheduledPowerEvent` |
| `IOPMCopyScheduledPowerEvents` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMCopyScheduledPowerEvents`; `iokit::copy_scheduled_power_events` |
| `kIOPMAssertPreventUserIdleSystemSleep` | macro | `pwr_mgt/IOPMLib.h` | `iokit::ffi::K_IOPMAssertPreventUserIdleSystemSleep`; `iokit::ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP` |
| `kIOPMAssertPreventUserIdleDisplaySleep` | macro | `pwr_mgt/IOPMLib.h` | `iokit::ffi::K_IOPMAssertPreventUserIdleDisplaySleep`; `iokit::ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP` |
| `kIOPMAssertPreventDiskIdle` | macro | `pwr_mgt/IOPMLib.h` | `iokit::ffi::K_IOPMAssertPreventDiskIdle`; `iokit::ASSERT_PREVENT_DISK_IDLE` |
| `kIOPMAssertNetworkClientActive` | macro | `pwr_mgt/IOPMLib.h` | `iokit::ffi::K_IOPMAssertNetworkClientActive`; `iokit::ASSERT_NETWORK_CLIENT_ACTIVE` |
| `IOPMAssertionID` | type | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionID`; `iokit::PowerAssertion` |
| `IOPMAssertionLevel` | type | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionLevel`; `iokit::PowerAssertionLevel` |
| `IOPMUserActiveType` | type | `pwr_mgt/IOPMLib.h` | `iokit::UserActiveType` |
| `IOPMAssertionCreateWithDescription` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionCreateWithDescription` |
| `IOPMAssertionCreateWithProperties` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionCreateWithProperties` |
| `IOPMAssertionDeclareUserActivity` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionDeclareUserActivity`; `iokit::PowerAssertion::declare_user_activity` |
| `IOPMDeclareNetworkClientActivity` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMDeclareNetworkClientActivity`; `iokit::PowerAssertion::declare_network_client_activity` |
| `IOPMAssertionRetain` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionRetain` |
| `IOPMAssertionRelease` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionRelease`; `iokit::PowerAssertion (Drop)` |
| `IOPMAssertionCopyProperties` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionCopyProperties`; `iokit::PowerAssertion::properties` |
| `IOPMAssertionSetProperty` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionSetProperty` |
| `IOPMCopyAssertionsByProcess` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMCopyAssertionsByProcess`; `iokit::copy_assertions_by_process` |
| `IOPMCopyAssertionsStatus` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMCopyAssertionsStatus`; `iokit::copy_assertions_status` |
| `IOPMAssertionCreate` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionCreate` |
| `IOPMAssertionCreateWithName` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMAssertionCreateWithName`; `iokit::PowerAssertion::create_with_level` |
| `kIOPMAssertionTypePreventUserIdleSystemSleep` | macro | `pwr_mgt/IOPMLib.h` | `iokit::ASSERT_PREVENT_USER_IDLE_SYSTEM_SLEEP` |
| `kIOPMAssertionTypePreventUserIdleDisplaySleep` | macro | `pwr_mgt/IOPMLib.h` | `iokit::ASSERT_PREVENT_USER_IDLE_DISPLAY_SLEEP` |
| `IOPMCopyCPUPowerStatus` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMCopyCPUPowerStatus`; `iokit::copy_cpu_power_status` |
| `IOPMGetThermalWarningLevel` | function | `pwr_mgt/IOPMLib.h` | `iokit::ffi::IOPMGetThermalWarningLevel`; `iokit::thermal_warning_level` |
| `kIOPSNotifyLowBattery` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPSNotifyLowBattery`; `iokit::NOTIFY_LOW_BATTERY` |
| `IOPSLowBatteryWarningLevel` | type | `ps/IOPowerSources.h` | `iokit::BatteryWarningLevel` |
| `IOPSGetBatteryWarningLevel` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSGetBatteryWarningLevel`; `iokit::battery_warning_level` |
| `kIOPSNotifyTimeRemaining` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPSNotifyTimeRemaining`; `iokit::NOTIFY_TIME_REMAINING` |
| `kIOPSTimeRemainingNotificationKey` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPSTimeRemainingNotificationKey`; `iokit::TIME_REMAINING_NOTIFICATION_KEY` |
| `kIOPSNotifyPowerSource` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPSNotifyPowerSource`; `iokit::NOTIFY_POWER_SOURCE` |
| `kIOPSNotifyAttach` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPSNotifyAttach`; `iokit::NOTIFY_ATTACH` |
| `kIOPSNotifyAnyPowerSource` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPSNotifyAnyPowerSource`; `iokit::NOTIFY_ANY_POWER_SOURCE` |
| `kIOPSTimeRemainingUnknown` | macro | `ps/IOPowerSources.h` | `iokit::ffi::kIOPSTimeRemainingUnknown`; `iokit::TIME_REMAINING_UNKNOWN` |
| `kIOPSTimeRemainingUnlimited` | macro | `ps/IOPowerSources.h` | `iokit::ffi::kIOPSTimeRemainingUnlimited`; `iokit::TIME_REMAINING_UNLIMITED` |
| `kIOPMUPSPowerKey` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPMUPSPowerKey`; `iokit::PROVIDING_POWER_UPS` |
| `kIOPMBatteryPowerKey` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPMBatteryPowerKey`; `iokit::PROVIDING_POWER_BATTERY` |
| `kIOPMACPowerKey` | macro | `ps/IOPowerSources.h` | `iokit::ffi::K_IOPMACPowerKey`; `iokit::PROVIDING_POWER_AC` |
| `IOPSGetTimeRemainingEstimate` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSGetTimeRemainingEstimate`; `iokit::time_remaining_estimate` |
| `IOPowerSourceCallbackType` | type | `ps/IOPowerSources.h` | `iokit::ffi::IOPowerSourceCallbackType` |
| `IOPSCopyPowerSourcesInfo` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSCopyPowerSourcesInfo`; `iokit::PowerSourcesInfo::copy` |
| `IOPSCopyPowerSourcesList` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSCopyPowerSourcesList`; `iokit::PowerSourcesInfo` |
| `IOPSGetPowerSourceDescription` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSGetPowerSourceDescription`; `iokit::PowerSourcesInfo::description` |
| `IOPSGetProvidingPowerSourceType` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSGetProvidingPowerSourceType`; `iokit::PowerSourcesInfo::providing_power_source_type` |
| `IOPSNotificationCreateRunLoopSource` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSNotificationCreateRunLoopSource` |
| `IOPSCreateLimitedPowerNotification` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSCreateLimitedPowerNotification` |
| `IOPSCopyExternalPowerAdapterDetails` | function | `ps/IOPowerSources.h` | `iokit::ffi::IOPSCopyExternalPowerAdapterDetails`; `iokit::copy_external_power_adapter_details` |
| `IOMessage` | type | `IOMessage.h` | `iokit::IoMessage` |
| `kIOMessageServiceIsTerminated` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceIsTerminated`; `iokit::IoMessage` |
| `kIOMessageServiceIsSuspended` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceIsSuspended`; `iokit::IoMessage` |
| `kIOMessageServiceIsResumed` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceIsResumed`; `iokit::IoMessage` |
| `kIOMessageServiceIsRequestingClose` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceIsRequestingClose`; `iokit::IoMessage` |
| `kIOMessageServiceIsAttemptingOpen` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceIsAttemptingOpen`; `iokit::IoMessage` |
| `kIOMessageServiceWasClosed` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceWasClosed`; `iokit::IoMessage` |
| `kIOMessageServiceBusyStateChange` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServiceBusyStateChange`; `iokit::IoMessage` |
| `kIOMessageConsoleSecurityChange` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageConsoleSecurityChange`; `iokit::IoMessage` |
| `kIOMessageServicePropertyChange` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageServicePropertyChange`; `iokit::IoMessage` |
| `kIOMessageCopyClientID` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageCopyClientID`; `iokit::IoMessage` |
| `kIOMessageSystemCapabilityChange` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemCapabilityChange`; `iokit::IoMessage` |
| `kIOMessageDeviceSignaledWakeup` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageDeviceSignaledWakeup`; `iokit::IoMessage` |
| `kIOMessageDeviceWillPowerOff` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageDeviceWillPowerOff`; `iokit::IoMessage` |
| `kIOMessageDeviceHasPoweredOn` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageDeviceHasPoweredOn`; `iokit::IoMessage` |
| `kIOMessageSystemWillPowerOff` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemWillPowerOff`; `iokit::IoMessage` |
| `kIOMessageSystemWillRestart` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemWillRestart`; `iokit::IoMessage` |
| `kIOMessageSystemPagingOff` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemPagingOff`; `iokit::IoMessage` |
| `kIOMessageCanSystemSleep` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageCanSystemSleep`; `iokit::IoMessage` |
| `kIOMessageSystemWillNotSleep` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemWillNotSleep`; `iokit::IoMessage` |
| `kIOMessageSystemWillSleep` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemWillSleep`; `iokit::IoMessage` |
| `kIOMessageSystemWillPowerOn` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemWillPowerOn`; `iokit::IoMessage` |
| `kIOMessageSystemHasPoweredOn` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageSystemHasPoweredOn`; `iokit::IoMessage` |
| `kIOMessageCanDevicePowerOff` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageCanDevicePowerOff`; `iokit::IoMessage` |
| `kIOMessageDeviceWillNotPowerOff` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageDeviceWillNotPowerOff`; `iokit::IoMessage` |
| `kIOMessageDeviceWillPowerOn` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageDeviceWillPowerOn`; `iokit::IoMessage` |
| `kIOMessageDeviceHasPoweredOff` | macro | `IOMessage.h` | `iokit::ffi::kIOMessageDeviceHasPoweredOff`; `iokit::IoMessage` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `kIOMainPortDefault` | const | `IOKitLib.h` | Modern main-port entry points are missing; the crate still leans on older master-port-era patterns. |
| `IOMainPort` | function | `IOKitLib.h` | Modern main-port entry points are missing; the crate still leans on older master-port-era patterns. |
| `IODispatchCalloutFromMessage` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IOCreateReceivePort` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IOKitGetBusyState` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IOKitWaitQuietWithOptions` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IOKitWaitQuiet` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IORegistryGetRootEntry` | function | `IOKitLib.h` | Registry helper is not exposed beyond the selected safe wrappers/raw-ffi subset. |
| `IORegistryCreateIterator` | function | `IOKitLib.h` | Registry helper is not exposed beyond the selected safe wrappers/raw-ffi subset. |
| `IORegistryIteratorEnterEntry` | function | `IOKitLib.h` | Registry helper is not exposed beyond the selected safe wrappers/raw-ffi subset. |
| `IORegistryIteratorExitEntry` | function | `IOKitLib.h` | Registry helper is not exposed beyond the selected safe wrappers/raw-ffi subset. |
| `IOBSDNameMatching` | function | `IOKitLib.h` | Additional matching helpers are not exposed. |
| `IOOpenFirmwarePathMatching` | function | `IOKitLib.h` | Additional matching helpers are not exposed. |
| `IOAsyncCallback0` | type | `IOKitLib.h` | Async callback typedef is not re-exported. |
| `IOAsyncCallback1` | type | `IOKitLib.h` | Async callback typedef is not re-exported. |
| `IOAsyncCallback2` | type | `IOKitLib.h` | Async callback typedef is not re-exported. |
| `IOAsyncCallback` | type | `IOKitLib.h` | Async callback typedef is not re-exported. |
| `IOCatalogueSendData` | function | `IOKitLib.h` | Catalogue-management APIs are not exposed. |
| `IOCatalogueTerminate` | function | `IOKitLib.h` | Catalogue-management APIs are not exposed. |
| `IOCatalogueGetData` | function | `IOKitLib.h` | Catalogue-management APIs are not exposed. |
| `IOCatalogueModuleLoaded` | function | `IOKitLib.h` | Catalogue-management APIs are not exposed. |
| `IOCatalogueReset` | function | `IOKitLib.h` | Catalogue-management APIs are not exposed. |
| `IOObject` | type | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IORegistryDisposeEnumerator` | function | `IOKitLib.h` | Registry helper is not exposed beyond the selected safe wrappers/raw-ffi subset. |
| `IOMapMemory` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `IOCompatibiltyNumber` | function | `IOKitLib.h` | Core IOKitLib helper is not exposed. |
| `kIOPMAssertionTimeoutKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTimeoutActionKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTimeoutActionLog` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTimeoutActionTurnOff` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTimeoutActionRelease` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionRetainCountKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionNameKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionDetailsKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionHumanReadableReasonKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionLocalizationBundlePathKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionFrameworkIDKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionPlugInIDKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTypeKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionLevelKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTypePreventSystemSleep` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTypeNoIdleSleep` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMAssertionTypeNoDisplaySleep` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOSystemLoadAdvisoryNotifyName` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `IOSystemLoadAdvisoryLevel` | type | `pwr_mgt/IOPMLib.h` | This IOPMLib entry point is not exposed. |
| `kIOSystemLoadAdvisoryUserLevelKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOSystemLoadAdvisoryBatteryLevelKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOSystemLoadAdvisoryThermalLevelKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOSystemLoadAdvisoryCombinedLevelKey` | macro | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `IOGetSystemLoadAdvisory` | function | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `IOCopySystemLoadAdvisoryDetailed` | function | `pwr_mgt/IOPMLib.h` | Assertion metadata/load-advisory APIs are omitted from the crate. |
| `kIOPMCPUPowerNotificationKey` | macro | `pwr_mgt/IOPMLib.h` | This IOPMLib entry point is not exposed. |
| `kIOPMThermalWarningNotificationKey` | macro | `pwr_mgt/IOPMLib.h` | This IOPMLib entry point is not exposed. |
| `kIOCFPlugInInterfaceID` | macro | `IOCFPlugIn.h` | Plug-in creation/destruction surface is not exposed. |
| `IOCFPlugInInterface` | type | `IOCFPlugIn.h` | Plug-in creation/destruction surface is not exposed. |
| `IOCreatePlugInInterfaceForService` | function | `IOCFPlugIn.h` | Plug-in creation/destruction surface is not exposed. |
| `IODestroyPlugInInterface` | function | `IOCFPlugIn.h` | Plug-in creation/destruction surface is not exposed. |
| `IOCFSerialize` | function | `IOCFSerialize.h` | CoreFoundation serialization helpers are not exposed. |
| `IOCFUnserialize` | function | `IOCFUnserialize.h` | CoreFoundation serialization helpers are not exposed. |
| `IOCFUnserializeBinary` | function | `IOCFUnserialize.h` | CoreFoundation serialization helpers are not exposed. |
| `IOCFUnserializeWithSize` | function | `IOCFUnserialize.h` | CoreFoundation serialization helpers are not exposed. |
| `IODataQueueDataAvailable` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `IODataQueuePeek` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `IODataQueueDequeue` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `IODataQueueWaitForAvailableData` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `IODataQueueAllocateNotificationPort` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `IODataQueueEnqueue` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `IODataQueueSetNotificationPort` | function | `IODataQueueClient.h` | Data-queue client helpers are not exposed. |
| `kIOUserServerClassKey` | macro | `IOUserServer.h` | IOUserServer key constants are not exposed. |
| `kIOUserServerNameKey` | macro | `IOUserServer.h` | IOUserServer key constants are not exposed. |
| `kIOUserServerTagKey` | macro | `IOUserServer.h` | IOUserServer key constants are not exposed. |
| `kIOUserServerCDHashKey` | macro | `IOUserServer.h` | IOUserServer key constants are not exposed. |
| `IOHIDManagerRef` | type | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerGetTypeID` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerCreate` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerOpen` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerClose` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerGetProperty` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetProperty` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerScheduleWithRunLoop` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerUnscheduleFromRunLoop` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetDispatchQueue` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetCancelHandler` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerActivate` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerCancel` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetDeviceMatching` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetDeviceMatchingMultiple` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerCopyDevices` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerRegisterDeviceMatchingCallback` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerRegisterDeviceRemovalCallback` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerRegisterInputReportCallback` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerRegisterInputReportWithTimeStampCallback` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerRegisterInputValueCallback` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetInputValueMatching` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSetInputValueMatchingMultiple` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDManagerSaveToPropertyDomain` | function | `hid/IOHIDManager.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetTypeID` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceCreate` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetService` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceOpen` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceClose` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceConformsTo` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetProperty` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetProperty` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceCopyMatchingElements` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceScheduleWithRunLoop` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceUnscheduleFromRunLoop` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetDispatchQueue` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetCancelHandler` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceActivate` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceCancel` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceRegisterRemovalCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceRegisterInputValueCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceRegisterInputReportCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceRegisterInputReportWithTimeStampCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetInputValueMatching` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetInputValueMatchingMultiple` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetValue` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetValueMultiple` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetValueWithCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetValueMultipleWithCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetValue` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetValueWithOptions` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceCopyValueMultiple` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetValueWithCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceCopyValueMultipleWithCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetReport` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceSetReportWithCallback` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |
| `IOHIDDeviceGetReport` | function | `hid/IOHIDDevice.h` | HID manager/device family is not exposed by the safe API or raw-ffi. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `kIOMasterPortDefault` | const | `IOKitLib.h` | Deprecated master-port alias | `__API_DEPRECATED_WITH_REPLACEMENT("kIOMainPortDefault", macos(10.0, 12.0))` |
| `IOMasterPort` | function | `IOKitLib.h` | Deprecated main-port lookup entry point | `__API_DEPRECATED_WITH_REPLACEMENT("kIOMainPortDefault", macos(10.0, 12.0))` |
| `IORegisterApp` | function | `pwr_mgt/IOPMLib.h` | Header marks this obsolete in favor of IOServiceAddInterestNotification | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_0, __MAC_10_9, __IPHONE_NA, __IPHONE_NA)` |
| `kIOMessageSystemWillNotPowerOff` | macro | `IOMessage.h` | Deprecated/unused message constant | `@deprecated in IOMessage.h docs` |
| `kIOMessageCanSystemPowerOff` | macro | `IOMessage.h` | Deprecated/unused message constant | `@deprecated in IOMessage.h docs` |

