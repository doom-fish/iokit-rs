//! Typed wrappers around public `IOMessage.h` constants.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{bridge, ffi_impl};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Typed wrapper around public `IOMessage.h` constants.
pub enum IoMessage {
    /// Wraps `kIOMessageCanDevicePowerOff`.
    CanDevicePowerOff,
    /// Wraps `kIOMessageCanSystemPowerOff`.
    CanSystemPowerOff,
    /// Wraps `kIOMessageCanSystemSleep`.
    CanSystemSleep,
    /// Wraps `kIOMessageConsoleSecurityChange`.
    ConsoleSecurityChange,
    /// Wraps `kIOMessageCopyClientID`.
    CopyClientId,
    /// Wraps `kIOMessageDeviceHasPoweredOff`.
    DeviceHasPoweredOff,
    /// Wraps `kIOMessageDeviceHasPoweredOn`.
    DeviceHasPoweredOn,
    /// Wraps `kIOMessageDeviceSignaledWakeup`.
    DeviceSignaledWakeup,
    /// Wraps `kIOMessageDeviceWillNotPowerOff`.
    DeviceWillNotPowerOff,
    /// Wraps `kIOMessageDeviceWillPowerOff`.
    DeviceWillPowerOff,
    /// Wraps `kIOMessageDeviceWillPowerOn`.
    DeviceWillPowerOn,
    /// Wraps `kIOMessageServiceBusyStateChange`.
    ServiceBusyStateChange,
    /// Wraps `kIOMessageServiceIsAttemptingOpen`.
    ServiceIsAttemptingOpen,
    /// Wraps `kIOMessageServiceIsRequestingClose`.
    ServiceIsRequestingClose,
    /// Wraps `kIOMessageServiceIsResumed`.
    ServiceIsResumed,
    /// Wraps `kIOMessageServiceIsSuspended`.
    ServiceIsSuspended,
    /// Wraps `kIOMessageServiceIsTerminated`.
    ServiceIsTerminated,
    /// Wraps `kIOMessageServicePropertyChange`.
    ServicePropertyChange,
    /// Wraps `kIOMessageServiceWasClosed`.
    ServiceWasClosed,
    /// Wraps `kIOMessageSystemCapabilityChange`.
    SystemCapabilityChange,
    /// Wraps `kIOMessageSystemHasPoweredOn`.
    SystemHasPoweredOn,
    /// Wraps `kIOMessageSystemPagingOff`.
    SystemPagingOff,
    /// Wraps `kIOMessageSystemWillNotPowerOff`.
    SystemWillNotPowerOff,
    /// Wraps `kIOMessageSystemWillNotSleep`.
    SystemWillNotSleep,
    /// Wraps `kIOMessageSystemWillPowerOff`.
    SystemWillPowerOff,
    /// Wraps `kIOMessageSystemWillPowerOn`.
    SystemWillPowerOn,
    /// Wraps `kIOMessageSystemWillRestart`.
    SystemWillRestart,
    /// Wraps `kIOMessageSystemWillSleep`.
    SystemWillSleep,
    /// Wraps an unrecognized message constant.
    Unknown(u32),
}

impl IoMessage {
    /// Builds an `IoMessage` from a raw `IOMessage.h` constant.
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            ffi_impl::kIOMessageCanDevicePowerOff => Self::CanDevicePowerOff,
            ffi_impl::kIOMessageCanSystemPowerOff => Self::CanSystemPowerOff,
            ffi_impl::kIOMessageCanSystemSleep => Self::CanSystemSleep,
            ffi_impl::kIOMessageConsoleSecurityChange => Self::ConsoleSecurityChange,
            ffi_impl::kIOMessageCopyClientID => Self::CopyClientId,
            ffi_impl::kIOMessageDeviceHasPoweredOff => Self::DeviceHasPoweredOff,
            ffi_impl::kIOMessageDeviceHasPoweredOn => Self::DeviceHasPoweredOn,
            ffi_impl::kIOMessageDeviceSignaledWakeup => Self::DeviceSignaledWakeup,
            ffi_impl::kIOMessageDeviceWillNotPowerOff => Self::DeviceWillNotPowerOff,
            ffi_impl::kIOMessageDeviceWillPowerOff => Self::DeviceWillPowerOff,
            ffi_impl::kIOMessageDeviceWillPowerOn => Self::DeviceWillPowerOn,
            ffi_impl::kIOMessageServiceBusyStateChange => Self::ServiceBusyStateChange,
            ffi_impl::kIOMessageServiceIsAttemptingOpen => Self::ServiceIsAttemptingOpen,
            ffi_impl::kIOMessageServiceIsRequestingClose => Self::ServiceIsRequestingClose,
            ffi_impl::kIOMessageServiceIsResumed => Self::ServiceIsResumed,
            ffi_impl::kIOMessageServiceIsSuspended => Self::ServiceIsSuspended,
            ffi_impl::kIOMessageServiceIsTerminated => Self::ServiceIsTerminated,
            ffi_impl::kIOMessageServicePropertyChange => Self::ServicePropertyChange,
            ffi_impl::kIOMessageServiceWasClosed => Self::ServiceWasClosed,
            ffi_impl::kIOMessageSystemCapabilityChange => Self::SystemCapabilityChange,
            ffi_impl::kIOMessageSystemHasPoweredOn => Self::SystemHasPoweredOn,
            ffi_impl::kIOMessageSystemPagingOff => Self::SystemPagingOff,
            ffi_impl::kIOMessageSystemWillNotPowerOff => Self::SystemWillNotPowerOff,
            ffi_impl::kIOMessageSystemWillNotSleep => Self::SystemWillNotSleep,
            ffi_impl::kIOMessageSystemWillPowerOff => Self::SystemWillPowerOff,
            ffi_impl::kIOMessageSystemWillPowerOn => Self::SystemWillPowerOn,
            ffi_impl::kIOMessageSystemWillRestart => Self::SystemWillRestart,
            ffi_impl::kIOMessageSystemWillSleep => Self::SystemWillSleep,
            other => Self::Unknown(other),
        }
    }

    /// Returns the raw `IOMessage.h` constant for this message.
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::CanDevicePowerOff => ffi_impl::kIOMessageCanDevicePowerOff,
            Self::CanSystemPowerOff => ffi_impl::kIOMessageCanSystemPowerOff,
            Self::CanSystemSleep => ffi_impl::kIOMessageCanSystemSleep,
            Self::ConsoleSecurityChange => ffi_impl::kIOMessageConsoleSecurityChange,
            Self::CopyClientId => ffi_impl::kIOMessageCopyClientID,
            Self::DeviceHasPoweredOff => ffi_impl::kIOMessageDeviceHasPoweredOff,
            Self::DeviceHasPoweredOn => ffi_impl::kIOMessageDeviceHasPoweredOn,
            Self::DeviceSignaledWakeup => ffi_impl::kIOMessageDeviceSignaledWakeup,
            Self::DeviceWillNotPowerOff => ffi_impl::kIOMessageDeviceWillNotPowerOff,
            Self::DeviceWillPowerOff => ffi_impl::kIOMessageDeviceWillPowerOff,
            Self::DeviceWillPowerOn => ffi_impl::kIOMessageDeviceWillPowerOn,
            Self::ServiceBusyStateChange => ffi_impl::kIOMessageServiceBusyStateChange,
            Self::ServiceIsAttemptingOpen => ffi_impl::kIOMessageServiceIsAttemptingOpen,
            Self::ServiceIsRequestingClose => ffi_impl::kIOMessageServiceIsRequestingClose,
            Self::ServiceIsResumed => ffi_impl::kIOMessageServiceIsResumed,
            Self::ServiceIsSuspended => ffi_impl::kIOMessageServiceIsSuspended,
            Self::ServiceIsTerminated => ffi_impl::kIOMessageServiceIsTerminated,
            Self::ServicePropertyChange => ffi_impl::kIOMessageServicePropertyChange,
            Self::ServiceWasClosed => ffi_impl::kIOMessageServiceWasClosed,
            Self::SystemCapabilityChange => ffi_impl::kIOMessageSystemCapabilityChange,
            Self::SystemHasPoweredOn => ffi_impl::kIOMessageSystemHasPoweredOn,
            Self::SystemPagingOff => ffi_impl::kIOMessageSystemPagingOff,
            Self::SystemWillNotPowerOff => ffi_impl::kIOMessageSystemWillNotPowerOff,
            Self::SystemWillNotSleep => ffi_impl::kIOMessageSystemWillNotSleep,
            Self::SystemWillPowerOff => ffi_impl::kIOMessageSystemWillPowerOff,
            Self::SystemWillPowerOn => ffi_impl::kIOMessageSystemWillPowerOn,
            Self::SystemWillRestart => ffi_impl::kIOMessageSystemWillRestart,
            Self::SystemWillSleep => ffi_impl::kIOMessageSystemWillSleep,
            Self::Unknown(other) => other,
        }
    }

    /// Returns every named message constant exposed by this crate.
    pub const fn all_known() -> &'static [Self] {
        const ALL: [IoMessage; 28] = [
            IoMessage::CanDevicePowerOff,
            IoMessage::CanSystemPowerOff,
            IoMessage::CanSystemSleep,
            IoMessage::ConsoleSecurityChange,
            IoMessage::CopyClientId,
            IoMessage::DeviceHasPoweredOff,
            IoMessage::DeviceHasPoweredOn,
            IoMessage::DeviceSignaledWakeup,
            IoMessage::DeviceWillNotPowerOff,
            IoMessage::DeviceWillPowerOff,
            IoMessage::DeviceWillPowerOn,
            IoMessage::ServiceBusyStateChange,
            IoMessage::ServiceIsAttemptingOpen,
            IoMessage::ServiceIsRequestingClose,
            IoMessage::ServiceIsResumed,
            IoMessage::ServiceIsSuspended,
            IoMessage::ServiceIsTerminated,
            IoMessage::ServicePropertyChange,
            IoMessage::ServiceWasClosed,
            IoMessage::SystemCapabilityChange,
            IoMessage::SystemHasPoweredOn,
            IoMessage::SystemPagingOff,
            IoMessage::SystemWillNotPowerOff,
            IoMessage::SystemWillNotSleep,
            IoMessage::SystemWillPowerOff,
            IoMessage::SystemWillPowerOn,
            IoMessage::SystemWillRestart,
            IoMessage::SystemWillSleep,
        ];
        &ALL
    }
}

/// Alias for `IoMessage` when handling system power callbacks.
pub type PowerMessage = IoMessage;

/// Builds a `PowerMessage` from a raw power callback code.
pub const fn power_message_from_raw(raw: u32) -> PowerMessage {
    PowerMessage::from_raw(raw)
}

/// Returns the number of message constants exposed by the Swift bridge.
pub fn bridged_constant_count() -> u32 {
    unsafe { bridge::iokit_swift_io_message_constant_count() }
}

/// Returns a bridged raw message constant by index.
pub fn bridged_constant(index: u32) -> u32 {
    unsafe { bridge::iokit_swift_io_message_constant(index) }
}
