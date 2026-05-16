//! Raw FFI declarations for selected public `IOKit` and `CoreFoundation` headers.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub type CFTypeRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFNumberRef = *const c_void;
pub type CFBooleanRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFMutableDictionaryRef = *mut c_void;
pub type CFRunLoopSourceRef = *const c_void;
pub type CFDateRef = *const c_void;
pub type CFIndex = isize;
pub type CFTypeID = usize;
pub type CFTimeInterval = f64;
pub type kern_return_t = i32;
pub type IOReturn = i32;
pub type IOOptionBits = u32;
pub type mach_port_t = u32;
pub type task_port_t = mach_port_t;
pub type boolean_t = u32;
pub type io_object_t = u32;
pub type io_service_t = u32;
pub type io_registry_entry_t = u32;
pub type io_iterator_t = u32;
pub type io_connect_t = u32;
pub type IOPMAssertionID = u32;
pub type IOPMAssertionLevel = u32;
pub type IONotificationPortRef = *mut c_void;
pub type dispatch_queue_t = *mut c_void;
pub type vm_address_t = usize;
pub type vm_size_t = usize;
pub type mach_vm_address_t = u64;
pub type mach_vm_size_t = u64;
pub type io_struct_inband_t = *mut c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mach_timespec_t {
    pub tv_sec: u32,
    pub tv_nsec: u32,
}

pub type IOServiceMatchingCallback =
    unsafe extern "C" fn(refcon: *mut c_void, iterator: io_iterator_t);
pub type IOServiceInterestCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    service: io_service_t,
    message_type: u32,
    message_argument: *mut c_void,
);
pub type IOPowerSourceCallbackType = unsafe extern "C" fn(context: *mut c_void);

pub const kIOReturnSuccess: IOReturn = 0;
pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;
pub const kCFNumberSInt64Type: i32 = 4;
pub const IO_NAME_SIZE: usize = 128;
pub const kIORegistryIterateRecursively: u32 = 0x0000_0001;
pub const kIORegistryIterateParents: u32 = 0x0000_0002;
pub const kIOServiceInteractionAllowed: u32 = 0x0000_0001;

pub const K_IO_SERVICE_PLANE: &str = "IOService";
pub const K_IOPublishNotification: &str = "IOServicePublish";
pub const K_IOFirstPublishNotification: &str = "IOServiceFirstPublish";
pub const K_IOMatchedNotification: &str = "IOServiceMatched";
pub const K_IOFirstMatchNotification: &str = "IOServiceFirstMatch";
pub const K_IOTerminatedNotification: &str = "IOServiceTerminate";
pub const K_IOGeneralInterest: &str = "IOGeneralInterest";
pub const K_IOBusyInterest: &str = "IOBusyInterest";

pub const kIOMessageCanDevicePowerOff: u32 = 3_758_096_896;
pub const kIOMessageCanSystemPowerOff: u32 = 3_758_096_960;
pub const kIOMessageCanSystemSleep: u32 = 3_758_097_008;
pub const kIOMessageConsoleSecurityChange: u32 = 3_758_096_680;
pub const kIOMessageCopyClientID: u32 = 3_758_097_200;
pub const kIOMessageDeviceHasPoweredOff: u32 = 3_758_096_933;
pub const kIOMessageDeviceHasPoweredOn: u32 = 3_758_096_944;
pub const kIOMessageDeviceSignaledWakeup: u32 = 3_758_097_232;
pub const kIOMessageDeviceWillNotPowerOff: u32 = 3_758_096_928;
pub const kIOMessageDeviceWillPowerOff: u32 = 3_758_096_912;
pub const kIOMessageDeviceWillPowerOn: u32 = 3_758_096_917;
pub const kIOMessageServiceBusyStateChange: u32 = 3_758_096_672;
pub const kIOMessageServiceIsAttemptingOpen: u32 = 3_758_096_641;
pub const kIOMessageServiceIsRequestingClose: u32 = 3_758_096_640;
pub const kIOMessageServiceIsResumed: u32 = 3_758_096_432;
pub const kIOMessageServiceIsSuspended: u32 = 3_758_096_416;
pub const kIOMessageServiceIsTerminated: u32 = 3_758_096_400;
pub const kIOMessageServicePropertyChange: u32 = 3_758_096_688;
pub const kIOMessageServiceWasClosed: u32 = 3_758_096_656;
pub const kIOMessageSystemCapabilityChange: u32 = 3_758_097_216;
pub const kIOMessageSystemHasPoweredOn: u32 = 3_758_097_152;
pub const kIOMessageSystemPagingOff: u32 = 3_758_096_981;
pub const kIOMessageSystemWillNotPowerOff: u32 = 3_758_096_992;
pub const kIOMessageSystemWillNotSleep: u32 = 3_758_097_040;
pub const kIOMessageSystemWillPowerOff: u32 = 3_758_096_976;
pub const kIOMessageSystemWillPowerOn: u32 = 3_758_097_184;
pub const kIOMessageSystemWillRestart: u32 = 3_758_097_168;
pub const kIOMessageSystemWillSleep: u32 = 3_758_097_024;

pub const K_IOPMAssertPreventUserIdleSystemSleep: &str = "PreventUserIdleSystemSleep";
pub const K_IOPMAssertPreventUserIdleDisplaySleep: &str = "PreventUserIdleDisplaySleep";
pub const K_IOPMAssertPreventDiskIdle: &str = "PreventDiskIdle";
pub const K_IOPMAssertNetworkClientActive: &str = "NetworkClientActive";
pub const K_IOPMAutoWake: &str = "wake";
pub const K_IOPMAutoPowerOn: &str = "poweron";
pub const K_IOPMAutoWakeOrPowerOn: &str = "wakepoweron";
pub const K_IOPMAutoSleep: &str = "sleep";
pub const K_IOPMAutoShutdown: &str = "shutdown";
pub const K_IOPMAutoRestart: &str = "restart";
pub const K_IOPMPowerEventTimeKey: &str = "time";
pub const K_IOPMPowerEventAppNameKey: &str = "scheduledby";
pub const K_IOPMPowerEventTypeKey: &str = "eventtype";
pub const K_IOPMCPUPowerLimitProcessorSpeedKey: &str = "CPU_Speed_Limit";
pub const K_IOPMCPUPowerLimitProcessorCountKey: &str = "CPU_Available_CPUs";
pub const K_IOPMCPUPowerLimitSchedulerTimeKey: &str = "CPU_Scheduler_Limit";
pub const kIOPMAssertionLevelOff: IOPMAssertionLevel = 0;
pub const kIOPMAssertionLevelOn: IOPMAssertionLevel = 255;
pub const kIOPMNullAssertionID: IOPMAssertionID = 0;
pub const kIOPMUserActiveLocal: u32 = 0;
pub const kIOPMUserActiveRemote: u32 = 1;
pub const kIOPMThermalWarningLevelNormal: u32 = 0;
pub const kIOPMThermalWarningLevelDanger: u32 = 100;
pub const kIOPMThermalWarningLevelCrisis: u32 = 10;

pub const K_IOPSNotifyLowBattery: &str = "com.apple.system.powersources.lowbattery";
pub const K_IOPSNotifyTimeRemaining: &str = "com.apple.system.powersources.timeremaining";
pub const K_IOPSTimeRemainingNotificationKey: &str = "com.apple.system.powersources.timeremaining";
pub const K_IOPSNotifyPowerSource: &str = "com.apple.system.powersources.source";
pub const K_IOPSNotifyAttach: &str = "com.apple.system.powersources.attach";
pub const K_IOPSNotifyAnyPowerSource: &str = "com.apple.system.powersources";
pub const K_IOPSPowerSourcesNotificationKey: &str = "com.apple.system.powersources";
pub const K_IOPMUPSPowerKey: &str = "UPS Power";
pub const K_IOPMBatteryPowerKey: &str = "Battery Power";
pub const K_IOPMACPowerKey: &str = "AC Power";
pub const kIOPSTimeRemainingUnknown: f64 = -1.0;
pub const kIOPSTimeRemainingUnlimited: f64 = -2.0;
pub const kIOPSLowBatteryWarningNone: u32 = 1;
pub const kIOPSLowBatteryWarningEarly: u32 = 2;
pub const kIOPSLowBatteryWarningFinal: u32 = 3;

unsafe extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;

    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetTypeID() -> CFTypeID;
    pub fn CFStringGetLength(string: CFStringRef) -> CFIndex;
    pub fn CFStringGetCString(
        string: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: u32,
    ) -> bool;

    pub fn CFNumberGetTypeID() -> CFTypeID;
    pub fn CFNumberGetValue(number: CFNumberRef, number_type: i32, value_ptr: *mut c_void) -> bool;

    pub fn CFBooleanGetTypeID() -> CFTypeID;
    pub fn CFBooleanGetValue(boolean: CFBooleanRef) -> bool;

    pub fn CFDataGetTypeID() -> CFTypeID;
    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;

    pub fn CFArrayGetTypeID() -> CFTypeID;
    pub fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(array: CFArrayRef, index: CFIndex) -> *const c_void;

    pub fn CFDictionaryGetTypeID() -> CFTypeID;
    pub fn CFDictionaryGetCount(dictionary: CFDictionaryRef) -> CFIndex;
    pub fn CFDictionaryGetKeysAndValues(
        dictionary: CFDictionaryRef,
        keys: *mut *const c_void,
        values: *mut *const c_void,
    );

    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;
    pub fn IOObjectRetain(object: io_object_t) -> kern_return_t;
    pub fn IOObjectGetClass(object: io_object_t, class_name: *mut c_char) -> kern_return_t;
    pub fn IOObjectCopyClass(object: io_object_t) -> CFStringRef;
    pub fn IOObjectCopySuperclassForClass(classname: CFStringRef) -> CFStringRef;
    pub fn IOObjectCopyBundleIdentifierForClass(classname: CFStringRef) -> CFStringRef;
    pub fn IOObjectConformsTo(object: io_object_t, class_name: *const c_char) -> boolean_t;
    pub fn IOObjectIsEqualTo(object: io_object_t, other: io_object_t) -> boolean_t;
    pub fn IOObjectGetKernelRetainCount(object: io_object_t) -> u32;
    pub fn IOObjectGetUserRetainCount(object: io_object_t) -> u32;
    pub fn IOObjectGetRetainCount(object: io_object_t) -> u32;

    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;
    pub fn IOIteratorReset(iterator: io_iterator_t);
    pub fn IOIteratorIsValid(iterator: io_iterator_t) -> boolean_t;

    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    pub fn IOServiceNameMatching(name: *const c_char) -> CFMutableDictionaryRef;
    pub fn IORegistryEntryIDMatching(entry_id: u64) -> CFMutableDictionaryRef;
    pub fn IOServiceGetMatchingService(
        main_port: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;
    pub fn IOServiceGetMatchingServices(
        main_port: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IOServiceAddNotification(
        main_port: mach_port_t,
        notification_type: *const c_char,
        matching: CFDictionaryRef,
        wake_port: mach_port_t,
        reference: usize,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IOServiceAddMatchingNotification(
        notify_port: IONotificationPortRef,
        notification_type: *const c_char,
        matching: CFDictionaryRef,
        callback: Option<IOServiceMatchingCallback>,
        refcon: *mut c_void,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IOServiceAddInterestNotification(
        notify_port: IONotificationPortRef,
        service: io_service_t,
        interest_type: *const c_char,
        callback: Option<IOServiceInterestCallback>,
        refcon: *mut c_void,
        notification: *mut io_object_t,
    ) -> kern_return_t;
    pub fn IOServiceMatchPropertyTable(
        service: io_service_t,
        matching: CFDictionaryRef,
        matches: *mut boolean_t,
    ) -> kern_return_t;
    pub fn IOServiceGetBusyState(service: io_service_t, busy_state: *mut u32) -> kern_return_t;
    pub fn IOServiceWaitQuiet(
        service: io_service_t,
        wait_time: *mut mach_timespec_t,
    ) -> kern_return_t;
    pub fn IOServiceOpen(
        service: io_service_t,
        owning_task: task_port_t,
        ty: u32,
        connect: *mut io_connect_t,
    ) -> kern_return_t;
    pub fn IOServiceRequestProbe(service: io_service_t, options: u32) -> kern_return_t;
    pub fn IOServiceAuthorize(service: io_service_t, options: u32) -> kern_return_t;
    pub fn IOServiceOpenAsFileDescriptor(service: io_service_t, oflag: i32) -> i32;
    pub fn IOServiceClose(connect: io_connect_t) -> kern_return_t;
    pub fn IOServiceOFPathToBSDName(
        main_port: mach_port_t,
        open_firmware_path: *const c_char,
        bsd_name: *mut c_char,
    ) -> kern_return_t;

    pub fn IONotificationPortCreate(main_port: mach_port_t) -> IONotificationPortRef;
    pub fn IONotificationPortDestroy(notify: IONotificationPortRef);
    pub fn IONotificationPortGetRunLoopSource(notify: IONotificationPortRef) -> CFRunLoopSourceRef;
    pub fn IONotificationPortGetMachPort(notify: IONotificationPortRef) -> mach_port_t;
    pub fn IONotificationPortSetImportanceReceiver(notify: IONotificationPortRef) -> kern_return_t;
    pub fn IONotificationPortSetDispatchQueue(
        notify: IONotificationPortRef,
        queue: dispatch_queue_t,
    );

    pub fn IORegistryEntryFromPath(
        main_port: mach_port_t,
        path: *const c_char,
    ) -> io_registry_entry_t;
    pub fn IORegistryEntryCopyFromPath(
        main_port: mach_port_t,
        path: CFStringRef,
    ) -> io_registry_entry_t;
    pub fn IORegistryEntryCreateIterator(
        entry: io_registry_entry_t,
        plane: *const c_char,
        options: IOOptionBits,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetName(entry: io_registry_entry_t, name: *mut c_char) -> kern_return_t;
    pub fn IORegistryEntryGetNameInPlane(
        entry: io_registry_entry_t,
        plane: *const c_char,
        name: *mut c_char,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetLocationInPlane(
        entry: io_registry_entry_t,
        plane: *const c_char,
        location: *mut c_char,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetPath(
        entry: io_registry_entry_t,
        plane: *const c_char,
        path: *mut c_char,
    ) -> kern_return_t;
    pub fn IORegistryEntryCopyPath(entry: io_registry_entry_t, plane: *const c_char)
        -> CFStringRef;
    pub fn IORegistryEntryGetRegistryEntryID(
        entry: io_registry_entry_t,
        entry_id: *mut u64,
    ) -> kern_return_t;
    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return_t;
    pub fn IORegistryEntryCreateCFProperty(
        entry: io_registry_entry_t,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;
    pub fn IORegistryEntrySearchCFProperty(
        entry: io_registry_entry_t,
        plane: *const c_char,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;
    pub fn IORegistryEntryGetProperty(
        entry: io_registry_entry_t,
        property_name: *const c_char,
        buffer: io_struct_inband_t,
        size: *mut u32,
    ) -> kern_return_t;
    pub fn IORegistryEntrySetCFProperties(
        entry: io_registry_entry_t,
        properties: CFTypeRef,
    ) -> kern_return_t;
    pub fn IORegistryEntrySetCFProperty(
        entry: io_registry_entry_t,
        property_name: CFStringRef,
        property: CFTypeRef,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetChildIterator(
        entry: io_registry_entry_t,
        plane: *const c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetChildEntry(
        entry: io_registry_entry_t,
        plane: *const c_char,
        child: *mut io_registry_entry_t,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetParentIterator(
        entry: io_registry_entry_t,
        plane: *const c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetParentEntry(
        entry: io_registry_entry_t,
        plane: *const c_char,
        parent: *mut io_registry_entry_t,
    ) -> kern_return_t;
    pub fn IORegistryEntryInPlane(entry: io_registry_entry_t, plane: *const c_char) -> boolean_t;

    pub fn IOConnectAddRef(connect: io_connect_t) -> kern_return_t;
    pub fn IOConnectRelease(connect: io_connect_t) -> kern_return_t;
    pub fn IOConnectGetService(connect: io_connect_t, service: *mut io_service_t) -> kern_return_t;
    pub fn IOConnectSetNotificationPort(
        connect: io_connect_t,
        ty: u32,
        port: mach_port_t,
        reference: usize,
    ) -> kern_return_t;
    pub fn IOConnectMapMemory(
        connect: io_connect_t,
        memory_type: u32,
        into_task: task_port_t,
        at_address: *mut mach_vm_address_t,
        of_size: *mut mach_vm_size_t,
        options: IOOptionBits,
    ) -> kern_return_t;
    pub fn IOConnectMapMemory64(
        connect: io_connect_t,
        memory_type: u32,
        into_task: task_port_t,
        at_address: *mut mach_vm_address_t,
        of_size: *mut mach_vm_size_t,
        options: IOOptionBits,
    ) -> kern_return_t;
    pub fn IOConnectUnmapMemory(
        connect: io_connect_t,
        memory_type: u32,
        from_task: task_port_t,
        at_address: mach_vm_address_t,
    ) -> kern_return_t;
    pub fn IOConnectUnmapMemory64(
        connect: io_connect_t,
        memory_type: u32,
        from_task: task_port_t,
        at_address: mach_vm_address_t,
    ) -> kern_return_t;
    pub fn IOConnectSetCFProperties(connect: io_connect_t, properties: CFTypeRef) -> kern_return_t;
    pub fn IOConnectSetCFProperty(
        connect: io_connect_t,
        property_name: CFStringRef,
        property: CFTypeRef,
    ) -> kern_return_t;
    pub fn IOConnectCallMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const u64,
        input_count: u32,
        input_struct: *const c_void,
        input_struct_count: usize,
        output: *mut u64,
        output_count: *mut u32,
        output_struct: *mut c_void,
        output_struct_count: *mut usize,
    ) -> kern_return_t;
    pub fn IOConnectCallAsyncMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        reference_count: u32,
        input: *const u64,
        input_count: u32,
        input_struct: *const c_void,
        input_struct_count: usize,
        output: *mut u64,
        output_count: *mut u32,
        output_struct: *mut c_void,
        output_struct_count: *mut usize,
    ) -> kern_return_t;
    pub fn IOConnectCallStructMethod(
        connection: mach_port_t,
        selector: u32,
        input_struct: *const c_void,
        input_struct_count: usize,
        output_struct: *mut c_void,
        output_struct_count: *mut usize,
    ) -> kern_return_t;
    pub fn IOConnectCallAsyncStructMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        reference_count: u32,
        input_struct: *const c_void,
        input_struct_count: usize,
        output_struct: *mut c_void,
        output_struct_count: *mut usize,
    ) -> kern_return_t;
    pub fn IOConnectCallScalarMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const u64,
        input_count: u32,
        output: *mut u64,
        output_count: *mut u32,
    ) -> kern_return_t;
    pub fn IOConnectCallAsyncScalarMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        reference_count: u32,
        input: *const u64,
        input_count: u32,
        output: *mut u64,
        output_count: *mut u32,
    ) -> kern_return_t;
    pub fn IOConnectTrap0(connect: io_connect_t, index: u32) -> kern_return_t;
    pub fn IOConnectTrap1(connect: io_connect_t, index: u32, p1: usize) -> kern_return_t;
    pub fn IOConnectTrap2(connect: io_connect_t, index: u32, p1: usize, p2: usize)
        -> kern_return_t;
    pub fn IOConnectTrap3(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
    ) -> kern_return_t;
    pub fn IOConnectTrap4(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) -> kern_return_t;
    pub fn IOConnectTrap5(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
        p5: usize,
    ) -> kern_return_t;
    pub fn IOConnectTrap6(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
        p5: usize,
        p6: usize,
    ) -> kern_return_t;
    pub fn IOConnectAddClient(connect: io_connect_t, client: io_connect_t) -> kern_return_t;
    pub fn IOConnectMethodScalarIScalarO(
        connect: io_connect_t,
        index: u32,
        scalar_input_count: u32,
        scalar_output_count: *mut u32,
        input: *const usize,
        output: *mut usize,
    ) -> kern_return_t;
    pub fn IOConnectMethodScalarIStructureO(
        connect: io_connect_t,
        index: u32,
        scalar_input_count: u32,
        structure_output_size: *mut usize,
        input: *const usize,
        output: *mut c_void,
    ) -> kern_return_t;
    pub fn IOConnectMethodScalarIStructureI(
        connect: io_connect_t,
        index: u32,
        scalar_input_count: u32,
        structure_input_size: usize,
        input: *const usize,
        structure_input: *const c_void,
    ) -> kern_return_t;
    pub fn IOConnectMethodStructureIStructureO(
        connect: io_connect_t,
        index: u32,
        structure_input_size: usize,
        structure_output_size: *mut usize,
        structure_input: *const c_void,
        structure_output: *mut c_void,
    ) -> kern_return_t;

    pub fn IOPMFindPowerManagement(master_device_port: mach_port_t) -> io_connect_t;
    pub fn IOPMSetAggressiveness(fb: io_connect_t, ty: usize, aggressiveness: usize) -> IOReturn;
    pub fn IOPMGetAggressiveness(
        fb: io_connect_t,
        ty: usize,
        aggressiveness: *mut usize,
    ) -> IOReturn;
    pub fn IOPMSleepEnabled() -> boolean_t;
    pub fn IOPMSleepSystem(fb: io_connect_t) -> IOReturn;
    pub fn IOPMCopyBatteryInfo(master_port: mach_port_t, info: *mut CFArrayRef) -> IOReturn;
    pub fn IORegisterApp(
        refcon: *mut c_void,
        the_driver: io_service_t,
        the_port_ref: *mut IONotificationPortRef,
        callback: Option<IOServiceInterestCallback>,
        notifier: *mut io_object_t,
    ) -> io_connect_t;
    pub fn IORegisterForSystemPower(
        refcon: *mut c_void,
        the_port_ref: *mut IONotificationPortRef,
        callback: Option<IOServiceInterestCallback>,
        notifier: *mut io_object_t,
    ) -> io_connect_t;
    pub fn IODeregisterApp(notifier: *mut io_object_t) -> IOReturn;
    pub fn IODeregisterForSystemPower(notifier: *mut io_object_t) -> IOReturn;
    pub fn IOAllowPowerChange(kernel_port: io_connect_t, notification_id: isize) -> IOReturn;
    pub fn IOCancelPowerChange(kernel_port: io_connect_t, notification_id: isize) -> IOReturn;
    pub fn IOPMSchedulePowerEvent(
        time_to_wake: CFDateRef,
        my_id: CFStringRef,
        event_type: CFStringRef,
    ) -> IOReturn;
    pub fn IOPMCancelScheduledPowerEvent(
        time_to_wake: CFDateRef,
        my_id: CFStringRef,
        event_type: CFStringRef,
    ) -> IOReturn;
    pub fn IOPMCopyScheduledPowerEvents() -> CFArrayRef;
    pub fn IOPMAssertionCopyProperties(assertion: IOPMAssertionID) -> CFDictionaryRef;
    pub fn IOPMAssertionCreate(
        assertion_type: CFStringRef,
        assertion_level: IOPMAssertionLevel,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMAssertionCreateWithDescription(
        assertion_type: CFStringRef,
        name: CFStringRef,
        details: CFStringRef,
        human_readable_reason: CFStringRef,
        localization_bundle_path: CFStringRef,
        timeout: CFTimeInterval,
        timeout_action: CFStringRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMAssertionCreateWithProperties(
        assertion_properties: CFDictionaryRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: IOPMAssertionLevel,
        assertion_name: CFStringRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMAssertionDeclareUserActivity(
        assertion_name: CFStringRef,
        user_type: u32,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMAssertionRelease(assertion_id: IOPMAssertionID) -> IOReturn;
    pub fn IOPMAssertionRetain(assertion_id: IOPMAssertionID);
    pub fn IOPMAssertionSetProperty(
        assertion: IOPMAssertionID,
        property: CFStringRef,
        value: CFTypeRef,
    ) -> IOReturn;
    pub fn IOPMCopyAssertionsByProcess(assertions_by_pid: *mut CFDictionaryRef) -> IOReturn;
    pub fn IOPMCopyAssertionsStatus(assertions_status: *mut CFDictionaryRef) -> IOReturn;
    pub fn IOPMCopyCPUPowerStatus(cpu_power_status: *mut CFDictionaryRef) -> IOReturn;
    pub fn IOPMDeclareNetworkClientActivity(
        assertion_name: CFStringRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMGetThermalWarningLevel(thermal_level: *mut u32) -> IOReturn;

    pub fn IOPSGetBatteryWarningLevel() -> u32;
    pub fn IOPSGetTimeRemainingEstimate() -> CFTimeInterval;
    pub fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
    pub fn IOPSCopyPowerSourcesList(blob: CFTypeRef) -> CFArrayRef;
    pub fn IOPSGetPowerSourceDescription(blob: CFTypeRef, ps: CFTypeRef) -> CFDictionaryRef;
    pub fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;
    pub fn IOPSNotificationCreateRunLoopSource(
        callback: Option<IOPowerSourceCallbackType>,
        context: *mut c_void,
    ) -> CFRunLoopSourceRef;
    pub fn IOPSCreateLimitedPowerNotification(
        callback: Option<IOPowerSourceCallbackType>,
        context: *mut c_void,
    ) -> CFRunLoopSourceRef;
    pub fn IOPSCopyExternalPowerAdapterDetails() -> CFDictionaryRef;
}
