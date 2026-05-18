#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    bridge,
    error::Result,
    io_notification_port::NotificationPort,
    io_service::Service,
    object::{io_result, nonnull},
};
use core::{ffi::c_void, ptr};
use std::ptr::NonNull;

#[derive(Debug)]
/// Combined output buffers returned by `IOConnectCallMethod`.
pub struct ConnectCallOutput {
    /// Scalar output values filled by `IOConnectCallMethod`.
    pub scalars: Vec<u64>,
    /// Structure output bytes filled by `IOConnectCallMethod`.
    pub structure: Vec<u8>,
}

#[derive(Debug)]
/// Safe retained wrapper around an `io_connect_t` handle returned by `IOServiceOpen`.
pub struct Connect {
    raw: NonNull<c_void>,
}

impl Connect {
    pub(crate) fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    /// Wraps `IOConnectGetService`.
    pub fn get_service(&self) -> Option<Service> {
        Service::from_raw(unsafe { bridge::iokit_swift_connect_get_service(self.as_ptr()) })
    }

    /// Wraps `IOConnectSetNotificationPort`.
    pub fn set_notification_port(
        &self,
        port: &NotificationPort,
        notification_type: u32,
        reference: usize,
    ) -> Result<()> {
        io_result(
            unsafe {
                bridge::iokit_swift_connect_set_notification_port(
                    self.as_ptr(),
                    port.as_ptr(),
                    notification_type,
                    reference,
                )
            },
            "IOConnectSetNotificationPort",
        )
    }

    /// Wraps `IOConnectAddClient`.
    pub fn add_client(&self, other: &Self) -> Result<()> {
        io_result(
            unsafe { bridge::iokit_swift_connect_add_client(self.as_ptr(), other.as_ptr()) },
            "IOConnectAddClient",
        )
    }

    /// Wraps `IOConnectCallScalarMethod`.
    pub fn call_scalar_method(
        &self,
        selector: u32,
        input: &[u64],
        output_capacity: u32,
    ) -> Result<Vec<u64>> {
        let mut output = vec![0_u64; usize::try_from(output_capacity).unwrap_or_default()];
        let mut output_count = output_capacity;
        io_result(
            unsafe {
                bridge::iokit_swift_connect_call_scalar_method(
                    self.as_ptr(),
                    selector,
                    input.as_ptr(),
                    u32::try_from(input.len()).unwrap_or_default(),
                    output.as_mut_ptr(),
                    &mut output_count,
                )
            },
            "IOConnectCallScalarMethod",
        )?;
        output.truncate(usize::try_from(output_count).unwrap_or_default());
        Ok(output)
    }

    /// Wraps `IOConnectCallStructMethod`.
    pub fn call_struct_method(
        &self,
        selector: u32,
        input: &[u8],
        output_capacity: usize,
    ) -> Result<Vec<u8>> {
        let mut output = vec![0_u8; output_capacity];
        let mut output_len = output_capacity;
        io_result(
            unsafe {
                bridge::iokit_swift_connect_call_struct_method(
                    self.as_ptr(),
                    selector,
                    if input.is_empty() {
                        ptr::null()
                    } else {
                        input.as_ptr().cast()
                    },
                    input.len(),
                    output.as_mut_ptr().cast(),
                    &mut output_len,
                )
            },
            "IOConnectCallStructMethod",
        )?;
        output.truncate(output_len);
        Ok(output)
    }

    /// Wraps `IOConnectCallMethod`.
    pub fn call_method(
        &self,
        selector: u32,
        input_scalars: &[u64],
        input_structure: &[u8],
        output_scalar_capacity: u32,
        output_structure_capacity: usize,
    ) -> Result<ConnectCallOutput> {
        let mut scalars = vec![0_u64; usize::try_from(output_scalar_capacity).unwrap_or_default()];
        let mut scalar_count = output_scalar_capacity;
        let mut structure = vec![0_u8; output_structure_capacity];
        let mut structure_len = output_structure_capacity;
        io_result(
            unsafe {
                bridge::iokit_swift_connect_call_method(
                    self.as_ptr(),
                    selector,
                    input_scalars.as_ptr(),
                    u32::try_from(input_scalars.len()).unwrap_or_default(),
                    if input_structure.is_empty() {
                        ptr::null()
                    } else {
                        input_structure.as_ptr().cast()
                    },
                    input_structure.len(),
                    scalars.as_mut_ptr(),
                    &mut scalar_count,
                    structure.as_mut_ptr().cast(),
                    &mut structure_len,
                )
            },
            "IOConnectCallMethod",
        )?;
        scalars.truncate(usize::try_from(scalar_count).unwrap_or_default());
        structure.truncate(structure_len);
        Ok(ConnectCallOutput { scalars, structure })
    }
}

impl Clone for Connect {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::iokit_swift_connect_retain(self.as_ptr()) };
        Self {
            raw: nonnull(raw, "iokit_swift_connect_retain").expect("connect retain"),
        }
    }
}

impl Drop for Connect {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_connect_release(self.as_ptr()) };
    }
}
