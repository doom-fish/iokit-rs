//! Raw FFI declarations for selected public `IOKit` and `CoreFoundation` headers.
//!
//! Pure C — no Swift bridge.

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
pub type CFIndex = isize;
pub type CFTypeID = usize;
pub type kern_return_t = i32;
pub type IOReturn = i32;
pub type IOOptionBits = u32;
pub type mach_port_t = u32;
pub type boolean_t = u32;
pub type io_object_t = u32;
pub type io_service_t = u32;
pub type io_registry_entry_t = u32;
pub type io_iterator_t = u32;
pub type io_connect_t = u32;
pub type IOPMAssertionID = u32;
pub type IOPMAssertionLevel = u32;
pub type IONotificationPortRef = *mut c_void;

pub type IOServiceMatchingCallback =
    unsafe extern "C" fn(refcon: *mut c_void, iterator: io_iterator_t);
pub type IOServiceInterestCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    service: io_service_t,
    message_type: u32,
    message_argument: *mut c_void,
);

pub const kIOReturnSuccess: IOReturn = 0;
pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;
pub const kCFNumberSInt64Type: i32 = 4;
pub const kIOPMAssertionLevelOff: IOPMAssertionLevel = 0;
pub const kIOPMAssertionLevelOn: IOPMAssertionLevel = 255;
pub const kIOMessageCanSystemSleep: u32 = 3_758_097_008;
pub const kIOMessageSystemWillSleep: u32 = 3_758_097_024;
pub const kIOMessageSystemWillNotSleep: u32 = 3_758_097_040;
pub const kIOMessageSystemHasPoweredOn: u32 = 3_758_097_152;
pub const kIOMessageSystemWillPowerOn: u32 = 3_758_097_184;
pub const IO_NAME_SIZE: usize = 128;
pub const K_IO_SERVICE_PLANE: &str = "IOService";

extern "C" {
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

    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;
    pub fn IOIteratorReset(iterator: io_iterator_t);
    pub fn IOIteratorIsValid(iterator: io_iterator_t) -> boolean_t;

    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;
    pub fn IOServiceGetMatchingService(
        main_port: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;
    pub fn IOServiceGetMatchingServices(
        main_port: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IOServiceAddMatchingNotification(
        notify_port: IONotificationPortRef,
        notification_type: *const c_char,
        matching: CFDictionaryRef,
        callback: Option<IOServiceMatchingCallback>,
        refcon: *mut c_void,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IONotificationPortCreate(main_port: mach_port_t) -> IONotificationPortRef;
    pub fn IONotificationPortDestroy(notify: IONotificationPortRef);
    pub fn IONotificationPortGetRunLoopSource(notify: IONotificationPortRef) -> CFRunLoopSourceRef;

    pub fn IORegistryEntryCreateCFProperty(
        entry: io_registry_entry_t,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;
    pub fn IORegistryEntryGetChildIterator(
        entry: io_registry_entry_t,
        plane: *const c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
    pub fn IORegistryEntryGetParentEntry(
        entry: io_registry_entry_t,
        plane: *const c_char,
        parent: *mut io_registry_entry_t,
    ) -> kern_return_t;

    pub fn IORegisterForSystemPower(
        refcon: *mut c_void,
        port_ref: *mut IONotificationPortRef,
        callback: Option<IOServiceInterestCallback>,
        notifier: *mut io_object_t,
    ) -> io_connect_t;
    pub fn IODeregisterForSystemPower(notifier: *mut io_object_t) -> kern_return_t;
    pub fn IOAllowPowerChange(kernel_port: io_connect_t, notification_id: isize) -> kern_return_t;
    pub fn IOCancelPowerChange(kernel_port: io_connect_t, notification_id: isize) -> kern_return_t;

    pub fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: IOPMAssertionLevel,
        assertion_name: CFStringRef,
        assertion_id: *mut IOPMAssertionID,
    ) -> IOReturn;
    pub fn IOPMAssertionRelease(assertion_id: IOPMAssertionID) -> IOReturn;
    pub fn IOPMCopyAssertionsByProcess(assertions_by_pid: *mut CFDictionaryRef) -> IOReturn;
}
