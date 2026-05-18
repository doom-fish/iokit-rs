//! Safe wrappers around global `IOKitLib` entry points.

#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    bridge,
    error::Result,
    ffi_impl,
    io_iterator::ObjectIterator,
    io_registry::RegistryEntry,
    io_service::Service,
    object::{c_string, io_result},
};
use core::ffi::{c_char, c_void};

/// Wraps `kIOMainPortDefault`.
pub const MAIN_PORT_DEFAULT: u32 = ffi_impl::kIOMainPortDefault;
/// Wraps `kIOCatalogAddDrivers`.
pub const CATALOG_ADD_DRIVERS: u32 = ffi_impl::kIOCatalogAddDrivers;
/// Wraps `kIOCatalogAddDriversNoMatch`.
pub const CATALOG_ADD_DRIVERS_NO_MATCH: u32 = ffi_impl::kIOCatalogAddDriversNoMatch;
/// Wraps `kIOCatalogRemoveDrivers`.
pub const CATALOG_REMOVE_DRIVERS: u32 = ffi_impl::kIOCatalogRemoveDrivers;
/// Wraps `kIOCatalogRemoveDriversNoMatch`.
pub const CATALOG_REMOVE_DRIVERS_NO_MATCH: u32 = ffi_impl::kIOCatalogRemoveDriversNoMatch;
/// Wraps `kIOCatalogKextdActive`.
pub const CATALOG_KEXTD_ACTIVE: u32 = ffi_impl::kIOCatalogKextdActive;
/// Wraps `kIOCatalogKextdFinishedLaunching`.
pub const CATALOG_KEXTD_FINISHED_LAUNCHING: u32 = ffi_impl::kIOCatalogKextdFinishedLaunching;
/// Wraps `kIOCatalogResetDrivers`.
pub const CATALOG_RESET_DRIVERS: u32 = ffi_impl::kIOCatalogResetDrivers;
/// Wraps `kIOCatalogResetDriversNoMatch`.
pub const CATALOG_RESET_DRIVERS_NO_MATCH: u32 = ffi_impl::kIOCatalogResetDriversNoMatch;
/// Wraps `kIOCatalogGetContents`.
pub const CATALOG_GET_CONTENTS: u32 = ffi_impl::kIOCatalogGetContents;
/// Wraps `kIOCatalogGetModuleDemandList`.
pub const CATALOG_GET_MODULE_DEMAND_LIST: u32 = ffi_impl::kIOCatalogGetModuleDemandList;
/// Wraps `kIOCatalogGetCacheMissList`.
pub const CATALOG_GET_CACHE_MISS_LIST: u32 = ffi_impl::kIOCatalogGetCacheMissList;
/// Wraps `kIOCatalogGetROMMkextList`.
pub const CATALOG_GET_ROM_MKEXT_LIST: u32 = ffi_impl::kIOCatalogGetROMMkextList;
/// Wraps `kIOCatalogResetDefault`.
pub const CATALOG_RESET_DEFAULT: u32 = ffi_impl::kIOCatalogResetDefault;
/// Wraps `kIOCatalogModuleUnload`.
pub const CATALOG_MODULE_UNLOAD: u32 = ffi_impl::kIOCatalogModuleUnload;
/// Wraps `kIOCatalogModuleTerminate`.
pub const CATALOG_MODULE_TERMINATE: u32 = ffi_impl::kIOCatalogModuleTerminate;
/// Wraps `kIOCatalogServiceTerminate`.
pub const CATALOG_SERVICE_TERMINATE: u32 = ffi_impl::kIOCatalogServiceTerminate;

fn wrap_service(raw: ffi_impl::io_service_t) -> Option<Service> {
    Service::from_raw(unsafe { bridge::iokit_swift_wrap_service(raw) })
}

fn wrap_registry_entry(raw: ffi_impl::io_registry_entry_t) -> Option<RegistryEntry> {
    RegistryEntry::from_raw(unsafe { bridge::iokit_swift_wrap_registry_entry(raw) })
}

fn wrap_iterator(raw: ffi_impl::io_iterator_t) -> Option<ObjectIterator> {
    ObjectIterator::from_raw(unsafe { bridge::iokit_swift_wrap_iterator(raw) })
}

/// Returns the default `IOMainPort`.
pub fn main_port() -> Result<u32> {
    main_port_from_bootstrap(MAIN_PORT_DEFAULT)
}

/// Wraps `IOMainPort` for an explicit bootstrap port.
pub fn main_port_from_bootstrap(bootstrap_port: u32) -> Result<u32> {
    let mut main_port = 0_u32;
    io_result(
        unsafe { ffi_impl::IOMainPort(bootstrap_port, &mut main_port) },
        "IOMainPort",
    )?;
    Ok(main_port)
}

/// Wraps `IOKitGetBusyState`.
pub fn kit_busy_state(main_port: u32) -> Result<u32> {
    let mut busy_state = 0_u32;
    io_result(
        unsafe { ffi_impl::IOKitGetBusyState(main_port, &mut busy_state) },
        "IOKitGetBusyState",
    )?;
    Ok(busy_state)
}

/// Wraps `IOKitWaitQuiet`.
pub fn kit_wait_quiet(main_port: u32, timeout_seconds: Option<u32>) -> Result<()> {
    let mut wait_time = ffi_impl::mach_timespec_t {
        tv_sec: timeout_seconds.unwrap_or_default(),
        tv_nsec: 0,
    };
    let wait_time = if timeout_seconds.is_some() {
        &mut wait_time
    } else {
        core::ptr::null_mut()
    };

    io_result(
        unsafe { ffi_impl::IOKitWaitQuiet(main_port, wait_time) },
        "IOKitWaitQuiet",
    )
}

/// Returns the root registry entry for `MAIN_PORT_DEFAULT`.
pub fn root_registry_entry() -> Option<RegistryEntry> {
    root_registry_entry_for_port(MAIN_PORT_DEFAULT)
}

/// Wraps `IORegistryGetRootEntry` for an explicit main port.
pub fn root_registry_entry_for_port(main_port: u32) -> Option<RegistryEntry> {
    wrap_registry_entry(unsafe { ffi_impl::IORegistryGetRootEntry(main_port) })
}

/// Wraps `IORegistryCreateIterator` for `MAIN_PORT_DEFAULT`.
pub fn registry_iterator(plane: &str, options: u32) -> Result<Option<ObjectIterator>> {
    registry_iterator_for_port(MAIN_PORT_DEFAULT, plane, options)
}

/// Wraps `IORegistryCreateIterator` for an explicit main port.
pub fn registry_iterator_for_port(
    main_port: u32,
    plane: &str,
    options: u32,
) -> Result<Option<ObjectIterator>> {
    let plane = c_string(plane)?;
    let mut iterator = 0_u32;
    io_result(
        unsafe {
            ffi_impl::IORegistryCreateIterator(main_port, plane.as_ptr(), options, &mut iterator)
        },
        "IORegistryCreateIterator",
    )?;
    Ok(wrap_iterator(iterator))
}

/// Finds the first service matching a BSD device name on `MAIN_PORT_DEFAULT`.
pub fn bsd_name_matching_service(bsd_name: &str) -> Result<Option<Service>> {
    bsd_name_matching_service_for_port(MAIN_PORT_DEFAULT, bsd_name)
}

/// Finds the first service matching a BSD device name for an explicit main port.
pub fn bsd_name_matching_service_for_port(
    main_port: u32,
    bsd_name: &str,
) -> Result<Option<Service>> {
    let bsd_name = c_string(bsd_name)?;
    let matching = unsafe { ffi_impl::IOBSDNameMatching(main_port, 0, bsd_name.as_ptr()) };
    if matching.is_null() {
        return Ok(None);
    }

    Ok(wrap_service(unsafe {
        ffi_impl::IOServiceGetMatchingService(main_port, matching)
    }))
}

/// Returns an iterator for services matching a BSD device name on `MAIN_PORT_DEFAULT`.
pub fn bsd_name_matching_services_iterator(bsd_name: &str) -> Result<Option<ObjectIterator>> {
    bsd_name_matching_services_iterator_for_port(MAIN_PORT_DEFAULT, bsd_name)
}

/// Returns an iterator for services matching a BSD device name for an explicit main port.
pub fn bsd_name_matching_services_iterator_for_port(
    main_port: u32,
    bsd_name: &str,
) -> Result<Option<ObjectIterator>> {
    let bsd_name = c_string(bsd_name)?;
    let matching = unsafe { ffi_impl::IOBSDNameMatching(main_port, 0, bsd_name.as_ptr()) };
    if matching.is_null() {
        return Ok(None);
    }

    let mut iterator = 0_u32;
    io_result(
        unsafe { ffi_impl::IOServiceGetMatchingServices(main_port, matching, &mut iterator) },
        "IOServiceGetMatchingServices",
    )?;
    Ok(wrap_iterator(iterator))
}

/// Collects all services matching a BSD device name.
pub fn bsd_name_matching_services(bsd_name: &str) -> Result<Vec<Service>> {
    Ok(bsd_name_matching_services_iterator(bsd_name)?
        .map_or_else(Vec::new, ObjectIterator::collect_services))
}

/// Wraps `IOCreateReceivePort`.
pub fn create_receive_port(msg_type: u32) -> Result<u32> {
    let mut recv_port = 0_u32;
    io_result(
        unsafe { ffi_impl::IOCreateReceivePort(msg_type, &mut recv_port) },
        "IOCreateReceivePort",
    )?;
    Ok(recv_port)
}

/// Wraps `IODispatchCalloutFromMessage` for a received Mach message.
pub unsafe fn dispatch_callout_from_message(
    msg: *mut ffi_impl::mach_msg_header_t,
    reference: *mut c_void,
) {
    unsafe { ffi_impl::IODispatchCalloutFromMessage(core::ptr::null_mut(), msg, reference) };
}

/// Wraps `IOCatalogueSendData` using `MAIN_PORT_DEFAULT`.
pub fn catalogue_send_data(flag: u32, buffer: &[u8]) -> Result<()> {
    let size = u32::try_from(buffer.len()).map_err(|_| {
        crate::IoKitError::InvalidArgument("catalogue payload exceeds u32::MAX bytes".to_string())
    })?;
    let buffer_ptr: *const c_char = if buffer.is_empty() {
        core::ptr::null()
    } else {
        buffer.as_ptr().cast()
    };

    io_result(
        unsafe { ffi_impl::IOCatalogueSendData(MAIN_PORT_DEFAULT, flag, buffer_ptr, size) },
        "IOCatalogueSendData",
    )
}

/// Wraps `IOCatalogueTerminate` using `MAIN_PORT_DEFAULT`.
pub fn catalogue_terminate(flag: u32, description: &str) -> Result<()> {
    let description = c_string(description)?;
    io_result(
        unsafe {
            ffi_impl::IOCatalogueTerminate(MAIN_PORT_DEFAULT, flag, description.as_ptr().cast_mut())
        },
        "IOCatalogueTerminate",
    )
}

/// Wraps `IOCatalogueGetData` using `MAIN_PORT_DEFAULT`.
pub unsafe fn catalogue_get_data_raw(
    flag: u32,
    buffer: *mut *mut c_char,
    size: *mut u32,
) -> Result<()> {
    io_result(
        unsafe { ffi_impl::IOCatalogueGetData(MAIN_PORT_DEFAULT, flag, buffer, size) },
        "IOCatalogueGetData",
    )
}

/// Wraps `IOCatalogueModuleLoaded` using `MAIN_PORT_DEFAULT`.
pub fn catalogue_module_loaded(name: &str) -> Result<()> {
    let name = c_string(name)?;
    io_result(
        unsafe { ffi_impl::IOCatalogueModuleLoaded(MAIN_PORT_DEFAULT, name.as_ptr().cast_mut()) },
        "IOCatalogueModuleLoaded",
    )
}

/// Wraps `IOCatalogueReset` using `MAIN_PORT_DEFAULT`.
pub fn catalogue_reset(flag: u32) -> Result<()> {
    io_result(
        unsafe { ffi_impl::IOCatalogueReset(MAIN_PORT_DEFAULT, flag) },
        "IOCatalogueReset",
    )
}
