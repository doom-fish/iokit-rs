#![allow(dead_code)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn iokit_swift_free_string(string: *mut c_char);

    pub fn iokit_swift_service_matching(name: *const c_char) -> *mut c_void;
    pub fn iokit_swift_service_name_matching(name: *const c_char) -> *mut c_void;
    pub fn iokit_swift_service_matching_entry_id(entry_id: u64) -> *mut c_void;
    pub fn iokit_swift_matching_services(name: *const c_char) -> *mut c_void;
    pub fn iokit_swift_name_matching_services(name: *const c_char) -> *mut c_void;
    pub fn iokit_swift_wrap_service(service: u32) -> *mut c_void;
    pub fn iokit_swift_service_raw(service: *mut c_void) -> u32;
    pub fn iokit_swift_service_retain(service: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_service_release(service: *mut c_void);
    pub fn iokit_swift_service_class_name(service: *mut c_void) -> *mut c_char;
    pub fn iokit_swift_service_bundle_identifier(service: *mut c_void) -> *mut c_char;
    pub fn iokit_swift_service_superclass_name(service: *mut c_void) -> *mut c_char;
    pub fn iokit_swift_service_conforms_to(service: *mut c_void, class_name: *const c_char)
        -> bool;
    pub fn iokit_swift_service_is_equal_to(service: *mut c_void, other: *mut c_void) -> bool;
    pub fn iokit_swift_service_kernel_retain_count(service: *mut c_void) -> u32;
    pub fn iokit_swift_service_user_retain_count(service: *mut c_void) -> u32;
    pub fn iokit_swift_service_retain_count(service: *mut c_void) -> u32;
    pub fn iokit_swift_service_busy_state(service: *mut c_void, busy_state: *mut u32) -> i32;
    pub fn iokit_swift_service_wait_quiet(service: *mut c_void, seconds: u32) -> i32;
    pub fn iokit_swift_service_authorize(service: *mut c_void, options: u32) -> i32;
    pub fn iokit_swift_service_open(
        service: *mut c_void,
        ty: u32,
        connect_out: *mut *mut c_void,
    ) -> i32;
    pub fn iokit_swift_service_as_registry_entry(service: *mut c_void) -> *mut c_void;

    pub fn iokit_swift_wrap_registry_entry(entry: u32) -> *mut c_void;
    pub fn iokit_swift_registry_entry_from_path(path: *const c_char) -> *mut c_void;
    pub fn iokit_swift_registry_entry_retain(entry: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_registry_entry_release(entry: *mut c_void);
    pub fn iokit_swift_registry_entry_name(entry: *mut c_void) -> *mut c_char;
    pub fn iokit_swift_registry_entry_name_in_plane(
        entry: *mut c_void,
        plane: *const c_char,
    ) -> *mut c_char;
    pub fn iokit_swift_registry_entry_location_in_plane(
        entry: *mut c_void,
        plane: *const c_char,
    ) -> *mut c_char;
    pub fn iokit_swift_registry_entry_path(entry: *mut c_void, plane: *const c_char)
        -> *mut c_char;
    pub fn iokit_swift_registry_entry_registry_entry_id(entry: *mut c_void, out: *mut u64) -> i32;
    pub fn iokit_swift_registry_entry_properties(entry: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_registry_entry_property(
        entry: *mut c_void,
        key: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_search_property(
        entry: *mut c_void,
        plane: *const c_char,
        key: *const c_char,
        options: u32,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_parent(
        entry: *mut c_void,
        plane: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_child(
        entry: *mut c_void,
        plane: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_parent_iterator(
        entry: *mut c_void,
        plane: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_child_iterator(
        entry: *mut c_void,
        plane: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_create_iterator(
        entry: *mut c_void,
        plane: *const c_char,
        options: u32,
    ) -> *mut c_void;
    pub fn iokit_swift_registry_entry_in_plane(entry: *mut c_void, plane: *const c_char) -> bool;

    pub fn iokit_swift_wrap_iterator(iterator: u32) -> *mut c_void;
    pub fn iokit_swift_iterator_retain(iterator: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_iterator_release(iterator: *mut c_void);
    pub fn iokit_swift_iterator_is_valid(iterator: *mut c_void) -> bool;
    pub fn iokit_swift_iterator_enter_entry(iterator: *mut c_void) -> i32;
    pub fn iokit_swift_iterator_exit_entry(iterator: *mut c_void) -> i32;
    pub fn iokit_swift_iterator_reset(iterator: *mut c_void);
    pub fn iokit_swift_iterator_next_service(iterator: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_iterator_next_registry_entry(iterator: *mut c_void) -> *mut c_void;

    pub fn iokit_swift_notification_port_create() -> *mut c_void;
    pub fn iokit_swift_notification_port_retain(port: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_notification_port_release(port: *mut c_void);
    pub fn iokit_swift_notification_port_mach_port(port: *mut c_void) -> u32;
    pub fn iokit_swift_notification_port_run_loop_source(port: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_notification_port_set_importance_receiver(port: *mut c_void) -> i32;

    pub fn iokit_swift_connect_retain(connect: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_connect_release(connect: *mut c_void);
    pub fn iokit_swift_connect_get_service(connect: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_connect_set_notification_port(
        connect: *mut c_void,
        port: *mut c_void,
        ty: u32,
        reference: usize,
    ) -> i32;
    pub fn iokit_swift_connect_add_client(connect: *mut c_void, client: *mut c_void) -> i32;
    pub fn iokit_swift_connect_call_scalar_method(
        connect: *mut c_void,
        selector: u32,
        input: *const u64,
        input_count: u32,
        output: *mut u64,
        output_count: *mut u32,
    ) -> i32;
    pub fn iokit_swift_connect_call_struct_method(
        connect: *mut c_void,
        selector: u32,
        input: *const c_void,
        input_len: usize,
        output: *mut c_void,
        output_len: *mut usize,
    ) -> i32;
    pub fn iokit_swift_connect_call_method(
        connect: *mut c_void,
        selector: u32,
        input_scalars: *const u64,
        input_scalar_count: u32,
        input_struct: *const c_void,
        input_struct_len: usize,
        output_scalars: *mut u64,
        output_scalar_count: *mut u32,
        output_struct: *mut c_void,
        output_struct_len: *mut usize,
    ) -> i32;

    pub fn iokit_swift_power_find_power_management() -> *mut c_void;
    pub fn iokit_swift_power_sleep_enabled() -> bool;
    pub fn iokit_swift_power_get_aggressiveness(
        connect: *mut c_void,
        ty: u64,
        value_out: *mut u64,
    ) -> i32;
    pub fn iokit_swift_power_set_aggressiveness(connect: *mut c_void, ty: u64, value: u64) -> i32;
    pub fn iokit_swift_power_get_thermal_warning_level(level_out: *mut u32) -> i32;
    pub fn iokit_swift_power_copy_assertions_by_process() -> *mut c_void;
    pub fn iokit_swift_power_copy_assertions_status() -> *mut c_void;
    pub fn iokit_swift_power_copy_battery_info() -> *mut c_void;
    pub fn iokit_swift_power_copy_cpu_power_status() -> *mut c_void;
    pub fn iokit_swift_power_copy_scheduled_power_events() -> *mut c_void;
    pub fn iokit_swift_power_assertion_create(
        assertion_type: *const c_char,
        level: u32,
        name: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_power_assertion_declare_user_activity(
        name: *const c_char,
        user_type: u32,
    ) -> *mut c_void;
    pub fn iokit_swift_power_assertion_declare_network_client_activity(
        name: *const c_char,
    ) -> *mut c_void;
    pub fn iokit_swift_power_assertion_retain(assertion: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_power_assertion_release(assertion: *mut c_void);
    pub fn iokit_swift_power_assertion_id(assertion: *mut c_void) -> u32;
    pub fn iokit_swift_power_assertion_copy_properties(assertion: *mut c_void) -> *mut c_void;

    pub fn iokit_swift_iops_power_sources_info_create() -> *mut c_void;
    pub fn iokit_swift_iops_power_sources_info_retain(info: *mut c_void) -> *mut c_void;
    pub fn iokit_swift_iops_power_sources_info_release(info: *mut c_void);
    pub fn iokit_swift_iops_power_sources_info_count(info: *mut c_void) -> usize;
    pub fn iokit_swift_iops_power_sources_info_description(
        info: *mut c_void,
        index: usize,
    ) -> *mut c_void;
    pub fn iokit_swift_iops_power_sources_info_provider_type(info: *mut c_void) -> *mut c_char;
    pub fn iokit_swift_iops_copy_external_power_adapter_details() -> *mut c_void;
    pub fn iokit_swift_iops_get_time_remaining_estimate() -> f64;
    pub fn iokit_swift_iops_get_battery_warning_level() -> u32;

    pub fn iokit_swift_io_message_constant_count() -> u32;
    pub fn iokit_swift_io_message_constant(index: u32) -> u32;

    pub fn iokit_swift_iohi_backing_store_public_sdk_available() -> bool;
    pub fn iokit_swift_iohi_backing_store_unavailability_reason() -> *mut c_char;

    // --- Async stream bridges ---

    /// Subscribe to service-interest notifications for `service`
    /// (`IOServiceAddInterestNotification`).
    /// Returns an opaque bridge handle on success, or null on failure.
    /// The caller must eventually call `iokit_swift_service_interest_unsubscribe`.
    pub fn iokit_swift_service_interest_subscribe(
        service: *mut c_void,
        interest_type: *const c_char,
        on_event: unsafe extern "C" fn(i32, *const c_void, *mut c_void),
        ctx: *mut c_void,
    ) -> *mut c_void;
    pub fn iokit_swift_service_interest_unsubscribe(handle: *mut c_void);

    /// Subscribe to service-match/terminate notifications for `class_name`.
    /// kind=0 → matched, kind=1 → terminated.  Payload is a retained
    /// `IOObjectHolder`\* (opaque pointer wrapping an `io_service_t`).
    pub fn iokit_swift_service_match_subscribe(
        class_name: *const c_char,
        on_event: unsafe extern "C" fn(i32, *const c_void, *mut c_void),
        ctx: *mut c_void,
    ) -> *mut c_void;
    pub fn iokit_swift_service_match_unsubscribe(handle: *mut c_void);

    /// Subscribe to power-source-changed notifications
    /// (`IOPSNotificationCreateRunLoopSource`).
    /// kind=0, payload=null; re-query IOPS for current state.
    pub fn iokit_swift_power_source_subscribe(
        on_event: unsafe extern "C" fn(i32, *const c_void, *mut c_void),
        ctx: *mut c_void,
    ) -> *mut c_void;
    pub fn iokit_swift_power_source_unsubscribe(handle: *mut c_void);

    /// Subscribe to system-power notifications (`IORegisterForSystemPower`).
    /// kind = `IOMessage` messageType; bridge auto-acknowledges
    /// `CanSystemSleep` / `SystemWillSleep` / `SystemWillPowerOff` /
    /// `SystemWillRestart`.
    pub fn iokit_swift_system_power_subscribe(
        on_event: unsafe extern "C" fn(i32, *const c_void, *mut c_void),
        ctx: *mut c_void,
    ) -> *mut c_void;
    pub fn iokit_swift_system_power_unsubscribe(handle: *mut c_void);
}
