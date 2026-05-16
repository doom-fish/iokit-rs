#![allow(clippy::missing_const_for_fn, clippy::used_underscore_binding)]

use core::ffi::{c_char, c_void};
use iokit::ffi;

unsafe extern "C" fn dummy_async0(_refcon: *mut c_void, _result: ffi::IOReturn) {}
unsafe extern "C" fn dummy_async1(
    _refcon: *mut c_void,
    _result: ffi::IOReturn,
    _arg0: *mut c_void,
) {
}
unsafe extern "C" fn dummy_async2(
    _refcon: *mut c_void,
    _result: ffi::IOReturn,
    _arg0: *mut c_void,
    _arg1: *mut c_void,
) {
}
unsafe extern "C" fn dummy_async(
    _refcon: *mut c_void,
    _result: ffi::IOReturn,
    _args: *mut *mut c_void,
    _num_args: u32,
) {
}

#[test]
fn exposes_low_level_raw_gap_surface() {
    let _ = ffi::K_IOUserServerClassKey;
    let _ = ffi::K_IOUserServerNameKey;
    let _ = ffi::K_IOUserServerTagKey;
    let _ = ffi::K_IOUserServerCDHashKey;
    let _ = ffi::kIOCFPlugInInterfaceID as unsafe fn() -> ffi::CFUUIDRef;
    let _ = core::mem::size_of::<ffi::IODataQueueEntry>();
    let _ = core::mem::size_of::<ffi::IODataQueueMemory>();
    let _ = core::mem::size_of::<ffi::IOCFPlugInInterface>();
    let _ = dummy_async0 as ffi::IOAsyncCallback0;
    let _ = dummy_async1 as ffi::IOAsyncCallback1;
    let _ = dummy_async2 as ffi::IOAsyncCallback2;
    let _ = dummy_async as ffi::IOAsyncCallback;

    let _plugin_create = ffi::IOCreatePlugInInterfaceForService
        as unsafe extern "C" fn(
            ffi::io_service_t,
            ffi::CFUUIDRef,
            ffi::CFUUIDRef,
            *mut *mut *mut ffi::IOCFPlugInInterface,
            *mut ffi::SInt32,
        ) -> ffi::kern_return_t;
    let _plugin_destroy = ffi::IODestroyPlugInInterface
        as unsafe extern "C" fn(*mut *mut ffi::IOCFPlugInInterface) -> ffi::kern_return_t;
    let _serialize = ffi::IOCFSerialize
        as unsafe extern "C" fn(ffi::CFTypeRef, ffi::CFOptionFlags) -> ffi::CFDataRef;
    let _unserialize = ffi::IOCFUnserialize
        as unsafe extern "C" fn(
            *const c_char,
            ffi::CFAllocatorRef,
            ffi::CFOptionFlags,
            *mut ffi::CFStringRef,
        ) -> ffi::CFTypeRef;
    let _unserialize_binary = ffi::IOCFUnserializeBinary
        as unsafe extern "C" fn(
            *const c_char,
            usize,
            ffi::CFAllocatorRef,
            ffi::CFOptionFlags,
            *mut ffi::CFStringRef,
        ) -> ffi::CFTypeRef;
    let _unserialize_with_size = ffi::IOCFUnserializeWithSize
        as unsafe extern "C" fn(
            *const c_char,
            usize,
            ffi::CFAllocatorRef,
            ffi::CFOptionFlags,
            *mut ffi::CFStringRef,
        ) -> ffi::CFTypeRef;

    let _data_available = ffi::IODataQueueDataAvailable
        as unsafe extern "C" fn(*mut ffi::IODataQueueMemory) -> u8;
    let _peek = ffi::IODataQueuePeek
        as unsafe extern "C" fn(*mut ffi::IODataQueueMemory) -> *mut ffi::IODataQueueEntry;
    let _dequeue = ffi::IODataQueueDequeue
        as unsafe extern "C" fn(*mut ffi::IODataQueueMemory, *mut c_void, *mut u32) -> ffi::IOReturn;
    let _wait = ffi::IODataQueueWaitForAvailableData
        as unsafe extern "C" fn(*mut ffi::IODataQueueMemory, ffi::mach_port_t) -> ffi::IOReturn;
    let _allocate_port = ffi::IODataQueueAllocateNotificationPort
        as unsafe extern "C" fn() -> ffi::mach_port_t;
    let _enqueue = ffi::IODataQueueEnqueue
        as unsafe extern "C" fn(*mut ffi::IODataQueueMemory, *mut c_void, u32) -> ffi::IOReturn;
    let _set_notification_port = ffi::IODataQueueSetNotificationPort
        as unsafe extern "C" fn(*mut ffi::IODataQueueMemory, ffi::mach_port_t) -> ffi::IOReturn;

    let _ = (
        _plugin_create,
        _plugin_destroy,
        _serialize,
        _unserialize,
        _unserialize_binary,
        _unserialize_with_size,
        _data_available,
        _peek,
        _dequeue,
        _wait,
        _allocate_port,
        _enqueue,
        _set_notification_port,
    );
}
