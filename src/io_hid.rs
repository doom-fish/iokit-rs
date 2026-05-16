#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::too_many_arguments
)]

use crate::{
    bridge,
    cf::take_value,
    error::Result,
    ffi_impl,
    io_service::Service,
    object::{c_string, io_result},
    CFValue,
};
use core::{ffi::c_void, ptr};
use std::ptr::NonNull;

pub const HID_MANAGER_OPTION_NONE: u32 = ffi_impl::kIOHIDManagerOptionNone;
pub const HID_MANAGER_OPTION_USE_PERSISTENT_PROPERTIES: u32 =
    ffi_impl::kIOHIDManagerOptionUsePersistentProperties;
pub const HID_MANAGER_OPTION_DO_NOT_LOAD_PROPERTIES: u32 =
    ffi_impl::kIOHIDManagerOptionDoNotLoadProperties;
pub const HID_MANAGER_OPTION_DO_NOT_SAVE_PROPERTIES: u32 =
    ffi_impl::kIOHIDManagerOptionDoNotSaveProperties;
pub const HID_MANAGER_OPTION_INDEPENDENT_DEVICES: u32 =
    ffi_impl::kIOHIDManagerOptionIndependentDevices;
pub const HID_DEVICE_GET_VALUE_WITH_UPDATE: u32 = ffi_impl::kIOHIDDeviceGetValueWithUpdate;
pub const HID_DEVICE_GET_VALUE_WITHOUT_UPDATE: u32 =
    ffi_impl::kIOHIDDeviceGetValueWithoutUpdate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HidReportType {
    Input,
    Output,
    Feature,
}

impl HidReportType {
    const fn as_raw(self) -> ffi_impl::IOHIDReportType {
        match self {
            Self::Input => ffi_impl::kIOHIDReportTypeInput,
            Self::Output => ffi_impl::kIOHIDReportTypeOutput,
            Self::Feature => ffi_impl::kIOHIDReportTypeFeature,
        }
    }
}

struct OwnedCfString {
    raw: ffi_impl::CFStringRef,
}

impl OwnedCfString {
    fn new(value: &str) -> Result<Self> {
        let value = c_string(value)?;
        let raw = unsafe {
            ffi_impl::CFStringCreateWithCString(
                ffi_impl::kCFAllocatorDefault,
                value.as_ptr(),
                ffi_impl::kCFStringEncodingUTF8,
            )
        };
        if raw.is_null() {
            return Err(crate::IoKitError::UnexpectedNull("CFStringCreateWithCString"));
        }
        Ok(Self { raw })
    }

    const fn as_raw(&self) -> ffi_impl::CFStringRef {
        self.raw
    }
}

impl Drop for OwnedCfString {
    fn drop(&mut self) {
        unsafe { ffi_impl::CFRelease(self.raw.cast()) };
    }
}

#[derive(Debug)]
pub struct HidManager {
    raw: NonNull<c_void>,
}

impl HidManager {
    fn from_retained_raw(raw: ffi_impl::IOHIDManagerRef) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(|raw| Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> ffi_impl::IOHIDManagerRef {
        self.raw.as_ptr().cast_const()
    }

    pub fn type_id() -> usize {
        unsafe { ffi_impl::IOHIDManagerGetTypeID() }
    }

    pub fn create(options: u32) -> Result<Self> {
        Self::from_retained_raw(unsafe {
            ffi_impl::IOHIDManagerCreate(ffi_impl::kCFAllocatorDefault, options)
        })
        .ok_or(crate::IoKitError::UnexpectedNull("IOHIDManagerCreate"))
    }

    pub fn open(&self, options: u32) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDManagerOpen(self.as_raw(), options) },
            "IOHIDManagerOpen",
        )
    }

    pub fn close(&self, options: u32) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDManagerClose(self.as_raw(), options) },
            "IOHIDManagerClose",
        )
    }

    pub fn property(&self, key: &str) -> Result<Option<CFValue>> {
        let key = OwnedCfString::new(key)?;
        let value = unsafe { ffi_impl::IOHIDManagerGetProperty(self.as_raw(), key.as_raw()) };
        if value.is_null() {
            return Ok(None);
        }
        unsafe {
            ffi_impl::CFRetain(value);
            Ok(take_value(value))
        }
    }

    pub unsafe fn set_property_raw(&self, key: &str, value: ffi_impl::CFTypeRef) -> Result<bool> {
        let key = OwnedCfString::new(key)?;
        Ok(unsafe { ffi_impl::IOHIDManagerSetProperty(self.as_raw(), key.as_raw(), value) != 0 })
    }

    pub unsafe fn schedule_with_run_loop_raw(
        &self,
        run_loop: ffi_impl::CFRunLoopRef,
        run_loop_mode: ffi_impl::CFStringRef,
    ) {
        unsafe { ffi_impl::IOHIDManagerScheduleWithRunLoop(self.as_raw(), run_loop, run_loop_mode) };
    }

    pub unsafe fn unschedule_from_run_loop_raw(
        &self,
        run_loop: ffi_impl::CFRunLoopRef,
        run_loop_mode: ffi_impl::CFStringRef,
    ) {
        unsafe {
            ffi_impl::IOHIDManagerUnscheduleFromRunLoop(self.as_raw(), run_loop, run_loop_mode);
        };
    }

    pub unsafe fn set_dispatch_queue_raw(&self, queue: ffi_impl::dispatch_queue_t) {
        unsafe { ffi_impl::IOHIDManagerSetDispatchQueue(self.as_raw(), queue) };
    }

    pub unsafe fn set_cancel_handler_raw(&self, handler: ffi_impl::dispatch_block_t) {
        unsafe { ffi_impl::IOHIDManagerSetCancelHandler(self.as_raw(), handler) };
    }

    pub fn activate(&self) {
        unsafe { ffi_impl::IOHIDManagerActivate(self.as_raw()) };
    }

    pub fn cancel(&self) {
        unsafe { ffi_impl::IOHIDManagerCancel(self.as_raw()) };
    }

    pub unsafe fn set_device_matching_raw(&self, matching: ffi_impl::CFDictionaryRef) {
        unsafe { ffi_impl::IOHIDManagerSetDeviceMatching(self.as_raw(), matching) };
    }

    pub unsafe fn set_device_matching_multiple_raw(&self, multiple: ffi_impl::CFArrayRef) {
        unsafe { ffi_impl::IOHIDManagerSetDeviceMatchingMultiple(self.as_raw(), multiple) };
    }

    pub fn devices(&self) -> Vec<HidDevice> {
        let device_set = unsafe { ffi_impl::IOHIDManagerCopyDevices(self.as_raw()) };
        if device_set.is_null() {
            return Vec::new();
        }

        let count = usize::try_from(unsafe { ffi_impl::CFSetGetCount(device_set) }).unwrap_or_default();
        let mut values = vec![ptr::null(); count];
        unsafe { ffi_impl::CFSetGetValues(device_set, values.as_mut_ptr()) };
        let devices = values
            .into_iter()
            .filter_map(|value| {
                if value.is_null() {
                    return None;
                }
                let retained = unsafe { ffi_impl::CFRetain(value.cast()) };
                HidDevice::from_retained_raw(retained.cast())
            })
            .collect();
        unsafe { ffi_impl::CFRelease(device_set.cast()) };
        devices
    }

    pub unsafe fn register_device_matching_callback(
        &self,
        callback: Option<ffi_impl::IOHIDDeviceCallback>,
        context: *mut c_void,
    ) {
        unsafe {
            ffi_impl::IOHIDManagerRegisterDeviceMatchingCallback(self.as_raw(), callback, context);
        };
    }

    pub unsafe fn register_device_removal_callback(
        &self,
        callback: Option<ffi_impl::IOHIDDeviceCallback>,
        context: *mut c_void,
    ) {
        unsafe {
            ffi_impl::IOHIDManagerRegisterDeviceRemovalCallback(self.as_raw(), callback, context);
        };
    }

    pub unsafe fn register_input_report_callback(
        &self,
        callback: Option<ffi_impl::IOHIDReportCallback>,
        context: *mut c_void,
    ) {
        unsafe {
            ffi_impl::IOHIDManagerRegisterInputReportCallback(self.as_raw(), callback, context);
        };
    }

    pub unsafe fn register_input_report_with_timestamp_callback(
        &self,
        callback: Option<ffi_impl::IOHIDReportWithTimeStampCallback>,
        context: *mut c_void,
    ) {
        unsafe {
            ffi_impl::IOHIDManagerRegisterInputReportWithTimeStampCallback(
                self.as_raw(),
                callback,
                context,
            );
        };
    }

    pub unsafe fn register_input_value_callback(
        &self,
        callback: Option<ffi_impl::IOHIDValueCallback>,
        context: *mut c_void,
    ) {
        unsafe { ffi_impl::IOHIDManagerRegisterInputValueCallback(self.as_raw(), callback, context) };
    }

    pub unsafe fn set_input_value_matching_raw(&self, matching: ffi_impl::CFDictionaryRef) {
        unsafe { ffi_impl::IOHIDManagerSetInputValueMatching(self.as_raw(), matching) };
    }

    pub unsafe fn set_input_value_matching_multiple_raw(&self, multiple: ffi_impl::CFArrayRef) {
        unsafe { ffi_impl::IOHIDManagerSetInputValueMatchingMultiple(self.as_raw(), multiple) };
    }

    pub fn save_to_property_domain(
        &self,
        application_id: &str,
        user_name: &str,
        host_name: &str,
        options: u32,
    ) -> Result<()> {
        let application_id = OwnedCfString::new(application_id)?;
        let user_name = OwnedCfString::new(user_name)?;
        let host_name = OwnedCfString::new(host_name)?;
        unsafe {
            ffi_impl::IOHIDManagerSaveToPropertyDomain(
                self.as_raw(),
                application_id.as_raw(),
                user_name.as_raw(),
                host_name.as_raw(),
                options,
            );
        };
        Ok(())
    }
}

impl Clone for HidManager {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi_impl::CFRetain(self.as_raw().cast()) };
        Self::from_retained_raw(raw.cast()).expect("hid manager retain")
    }
}

impl Drop for HidManager {
    fn drop(&mut self) {
        unsafe { ffi_impl::CFRelease(self.as_raw().cast()) };
    }
}

#[derive(Debug)]
pub struct HidDevice {
    raw: NonNull<c_void>,
}

impl HidDevice {
    fn from_retained_raw(raw: ffi_impl::IOHIDDeviceRef) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(|raw| Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> ffi_impl::IOHIDDeviceRef {
        self.raw.as_ptr().cast_const()
    }

    pub fn type_id() -> usize {
        unsafe { ffi_impl::IOHIDDeviceGetTypeID() }
    }

    pub fn create(service: &Service) -> Result<Self> {
        Self::create_from_service(unsafe { bridge::iokit_swift_service_raw(service.as_ptr()) })
    }

    pub fn create_from_service(service: ffi_impl::io_service_t) -> Result<Self> {
        Self::from_retained_raw(unsafe {
            ffi_impl::IOHIDDeviceCreate(ffi_impl::kCFAllocatorDefault, service)
        })
        .ok_or(crate::IoKitError::UnexpectedNull("IOHIDDeviceCreate"))
    }

    pub fn service(&self) -> Option<Service> {
        let service = unsafe { ffi_impl::IOHIDDeviceGetService(self.as_raw()) };
        if service == 0 {
            return None;
        }
        if unsafe { ffi_impl::IOObjectRetain(service) } != 0 {
            return None;
        }
        Service::from_raw(unsafe { bridge::iokit_swift_wrap_service(service) })
    }

    pub fn open(&self, options: u32) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDDeviceOpen(self.as_raw(), options) },
            "IOHIDDeviceOpen",
        )
    }

    pub fn close(&self, options: u32) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDDeviceClose(self.as_raw(), options) },
            "IOHIDDeviceClose",
        )
    }

    pub fn conforms_to(&self, usage_page: u32, usage: u32) -> bool {
        unsafe { ffi_impl::IOHIDDeviceConformsTo(self.as_raw(), usage_page, usage) != 0 }
    }

    pub fn property(&self, key: &str) -> Result<Option<CFValue>> {
        let key = OwnedCfString::new(key)?;
        let value = unsafe { ffi_impl::IOHIDDeviceGetProperty(self.as_raw(), key.as_raw()) };
        if value.is_null() {
            return Ok(None);
        }
        unsafe {
            ffi_impl::CFRetain(value);
            Ok(take_value(value))
        }
    }

    pub unsafe fn set_property_raw(&self, key: &str, property: ffi_impl::CFTypeRef) -> Result<bool> {
        let key = OwnedCfString::new(key)?;
        Ok(unsafe { ffi_impl::IOHIDDeviceSetProperty(self.as_raw(), key.as_raw(), property) != 0 })
    }

    pub unsafe fn matching_elements_raw(
        &self,
        matching: ffi_impl::CFDictionaryRef,
        options: u32,
    ) -> Option<CFValue> {
        unsafe {
            take_value(ffi_impl::IOHIDDeviceCopyMatchingElements(self.as_raw(), matching, options).cast())
        }
    }

    pub fn all_matching_elements(&self) -> Option<CFValue> {
        unsafe { self.matching_elements_raw(ptr::null(), 0) }
    }

    pub unsafe fn schedule_with_run_loop_raw(
        &self,
        run_loop: ffi_impl::CFRunLoopRef,
        run_loop_mode: ffi_impl::CFStringRef,
    ) {
        unsafe { ffi_impl::IOHIDDeviceScheduleWithRunLoop(self.as_raw(), run_loop, run_loop_mode) };
    }

    pub unsafe fn unschedule_from_run_loop_raw(
        &self,
        run_loop: ffi_impl::CFRunLoopRef,
        run_loop_mode: ffi_impl::CFStringRef,
    ) {
        unsafe {
            ffi_impl::IOHIDDeviceUnscheduleFromRunLoop(self.as_raw(), run_loop, run_loop_mode);
        };
    }

    pub unsafe fn set_dispatch_queue_raw(&self, queue: ffi_impl::dispatch_queue_t) {
        unsafe { ffi_impl::IOHIDDeviceSetDispatchQueue(self.as_raw(), queue) };
    }

    pub unsafe fn set_cancel_handler_raw(&self, handler: ffi_impl::dispatch_block_t) {
        unsafe { ffi_impl::IOHIDDeviceSetCancelHandler(self.as_raw(), handler) };
    }

    pub fn activate(&self) {
        unsafe { ffi_impl::IOHIDDeviceActivate(self.as_raw()) };
    }

    pub fn cancel(&self) {
        unsafe { ffi_impl::IOHIDDeviceCancel(self.as_raw()) };
    }

    pub unsafe fn register_removal_callback(
        &self,
        callback: Option<ffi_impl::IOHIDCallback>,
        context: *mut c_void,
    ) {
        unsafe { ffi_impl::IOHIDDeviceRegisterRemovalCallback(self.as_raw(), callback, context) };
    }

    pub unsafe fn register_input_value_callback(
        &self,
        callback: Option<ffi_impl::IOHIDValueCallback>,
        context: *mut c_void,
    ) {
        unsafe { ffi_impl::IOHIDDeviceRegisterInputValueCallback(self.as_raw(), callback, context) };
    }

    pub unsafe fn register_input_report_callback(
        &self,
        report: *mut u8,
        report_length: isize,
        callback: Option<ffi_impl::IOHIDReportCallback>,
        context: *mut c_void,
    ) {
        unsafe {
            ffi_impl::IOHIDDeviceRegisterInputReportCallback(
                self.as_raw(),
                report,
                report_length,
                callback,
                context,
            );
        };
    }

    pub unsafe fn register_input_report_with_timestamp_callback(
        &self,
        report: *mut u8,
        report_length: isize,
        callback: Option<ffi_impl::IOHIDReportWithTimeStampCallback>,
        context: *mut c_void,
    ) {
        unsafe {
            ffi_impl::IOHIDDeviceRegisterInputReportWithTimeStampCallback(
                self.as_raw(),
                report,
                report_length,
                callback,
                context,
            );
        };
    }

    pub unsafe fn set_input_value_matching_raw(&self, matching: ffi_impl::CFDictionaryRef) {
        unsafe { ffi_impl::IOHIDDeviceSetInputValueMatching(self.as_raw(), matching) };
    }

    pub unsafe fn set_input_value_matching_multiple_raw(&self, multiple: ffi_impl::CFArrayRef) {
        unsafe { ffi_impl::IOHIDDeviceSetInputValueMatchingMultiple(self.as_raw(), multiple) };
    }

    pub unsafe fn set_value_raw(
        &self,
        element: ffi_impl::IOHIDElementRef,
        value: ffi_impl::IOHIDValueRef,
    ) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDDeviceSetValue(self.as_raw(), element, value) },
            "IOHIDDeviceSetValue",
        )
    }

    pub unsafe fn set_value_multiple_raw(&self, multiple: ffi_impl::CFDictionaryRef) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDDeviceSetValueMultiple(self.as_raw(), multiple) },
            "IOHIDDeviceSetValueMultiple",
        )
    }

    pub unsafe fn set_value_with_callback_raw(
        &self,
        element: ffi_impl::IOHIDElementRef,
        value: ffi_impl::IOHIDValueRef,
        timeout: f64,
        callback: Option<ffi_impl::IOHIDValueCallback>,
        context: *mut c_void,
    ) -> Result<()> {
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceSetValueWithCallback(
                    self.as_raw(),
                    element,
                    value,
                    timeout,
                    callback,
                    context,
                )
            },
            "IOHIDDeviceSetValueWithCallback",
        )
    }

    pub unsafe fn set_value_multiple_with_callback_raw(
        &self,
        multiple: ffi_impl::CFDictionaryRef,
        timeout: f64,
        callback: Option<ffi_impl::IOHIDValueMultipleCallback>,
        context: *mut c_void,
    ) -> Result<()> {
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceSetValueMultipleWithCallback(
                    self.as_raw(),
                    multiple,
                    timeout,
                    callback,
                    context,
                )
            },
            "IOHIDDeviceSetValueMultipleWithCallback",
        )
    }

    pub unsafe fn get_value_raw(
        &self,
        element: ffi_impl::IOHIDElementRef,
        value_out: *mut ffi_impl::IOHIDValueRef,
    ) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDDeviceGetValue(self.as_raw(), element, value_out) },
            "IOHIDDeviceGetValue",
        )
    }

    pub unsafe fn get_value_with_options_raw(
        &self,
        element: ffi_impl::IOHIDElementRef,
        value_out: *mut ffi_impl::IOHIDValueRef,
        options: u32,
    ) -> Result<()> {
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceGetValueWithOptions(self.as_raw(), element, value_out, options)
            },
            "IOHIDDeviceGetValueWithOptions",
        )
    }

    pub unsafe fn copy_value_multiple_raw(
        &self,
        elements: ffi_impl::CFArrayRef,
        multiple_out: *mut ffi_impl::CFDictionaryRef,
    ) -> Result<()> {
        io_result(
            unsafe { ffi_impl::IOHIDDeviceCopyValueMultiple(self.as_raw(), elements, multiple_out) },
            "IOHIDDeviceCopyValueMultiple",
        )
    }

    pub unsafe fn get_value_with_callback_raw(
        &self,
        element: ffi_impl::IOHIDElementRef,
        value_out: *mut ffi_impl::IOHIDValueRef,
        timeout: f64,
        callback: Option<ffi_impl::IOHIDValueCallback>,
        context: *mut c_void,
    ) -> Result<()> {
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceGetValueWithCallback(
                    self.as_raw(),
                    element,
                    value_out,
                    timeout,
                    callback,
                    context,
                )
            },
            "IOHIDDeviceGetValueWithCallback",
        )
    }

    pub unsafe fn copy_value_multiple_with_callback_raw(
        &self,
        elements: ffi_impl::CFArrayRef,
        multiple_out: *mut ffi_impl::CFDictionaryRef,
        timeout: f64,
        callback: Option<ffi_impl::IOHIDValueMultipleCallback>,
        context: *mut c_void,
    ) -> Result<()> {
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceCopyValueMultipleWithCallback(
                    self.as_raw(),
                    elements,
                    multiple_out,
                    timeout,
                    callback,
                    context,
                )
            },
            "IOHIDDeviceCopyValueMultipleWithCallback",
        )
    }

    pub fn set_report(
        &self,
        report_type: HidReportType,
        report_id: isize,
        report: &[u8],
    ) -> Result<()> {
        let report_length = isize::try_from(report.len()).map_err(|_| {
            crate::IoKitError::InvalidArgument("report buffer exceeds CFIndex::MAX bytes".to_string())
        })?;
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceSetReport(
                    self.as_raw(),
                    report_type.as_raw(),
                    report_id,
                    report.as_ptr(),
                    report_length,
                )
            },
            "IOHIDDeviceSetReport",
        )
    }

    pub unsafe fn set_report_with_callback_raw(
        &self,
        report_type: HidReportType,
        report_id: isize,
        report: *const u8,
        report_length: isize,
        timeout: f64,
        callback: Option<ffi_impl::IOHIDReportCallback>,
        context: *mut c_void,
    ) -> Result<()> {
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceSetReportWithCallback(
                    self.as_raw(),
                    report_type.as_raw(),
                    report_id,
                    report,
                    report_length,
                    timeout,
                    callback,
                    context,
                )
            },
            "IOHIDDeviceSetReportWithCallback",
        )
    }

    pub fn get_report(
        &self,
        report_type: HidReportType,
        report_id: isize,
        report: &mut [u8],
    ) -> Result<usize> {
        let mut report_length = isize::try_from(report.len()).map_err(|_| {
            crate::IoKitError::InvalidArgument("report buffer exceeds CFIndex::MAX bytes".to_string())
        })?;
        io_result(
            unsafe {
                ffi_impl::IOHIDDeviceGetReport(
                    self.as_raw(),
                    report_type.as_raw(),
                    report_id,
                    report.as_mut_ptr(),
                    &mut report_length,
                )
            },
            "IOHIDDeviceGetReport",
        )?;
        Ok(usize::try_from(report_length).unwrap_or_default())
    }
}

impl Clone for HidDevice {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi_impl::CFRetain(self.as_raw().cast()) };
        Self::from_retained_raw(raw.cast()).expect("hid device retain")
    }
}

impl Drop for HidDevice {
    fn drop(&mut self) {
        unsafe { ffi_impl::CFRelease(self.as_raw().cast()) };
    }
}
