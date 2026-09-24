//! Safe wrappers around `IONotificationPort*` APIs.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use crate::{
    bridge,
    cf::dictionary_to_cf,
    error::Result,
    ffi_impl,
    io_message::IoMessage,
    io_service::{ExistingServices, MatchingDictionary, Service},
    object::{c_string, io_result, nonnull},
    IoKitError,
};
pub use apple_cf::dispatch_queue::{DispatchQoS, DispatchQueue};
use core::ffi::c_void;
use doom_fish_utils::{callback_context::CallbackContext, panic_safe::catch_user_panic};
use std::{
    ptr::NonNull,
    sync::{Mutex, PoisonError},
};

/// An `IOKit` service-interest event, delivered by `NotificationPort::add_interest_notification`
/// and `async_api::ServiceInterestStream`.
#[allow(clippy::doc_markdown)]
#[derive(Debug, Clone)]
pub struct ServiceInterestEvent {
    /// Decoded IOKit message type (e.g. `IoMessage::ServiceBusyStateChange`).
    pub message: IoMessage,
    /// Raw message-argument pointer value (may be 0 / null).
    /// Interpretation is message-type–specific; see IOKit documentation.
    pub message_argument: usize,
}

enum Handler {
    Matching(Box<dyn FnMut(Service) + Send>),
    Interest(Box<dyn FnMut(ServiceInterestEvent) + Send>),
}

type PortContext = CallbackContext<Mutex<Handler>>;

#[derive(Debug)]
/// Safe retained wrapper around an `IONotificationPortRef`.
pub struct NotificationPort {
    raw: NonNull<c_void>,
}

unsafe impl Send for NotificationPort {}
unsafe impl Sync for NotificationPort {}

impl NotificationPort {
    /// Wraps `IONotificationPortCreate`.
    pub fn new() -> Result<Self> {
        let raw = unsafe { bridge::iokit_swift_notification_port_create() };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_notification_port_create")?,
        })
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }

    fn retained(&self) -> Result<Self> {
        let raw = unsafe { bridge::iokit_swift_notification_port_retain(self.as_ptr()) };
        Ok(Self {
            raw: nonnull(raw, "iokit_swift_notification_port_retain")?,
        })
    }

    /// Returns the Mach port backing this notification port.
    pub fn mach_port(&self) -> u32 {
        unsafe { bridge::iokit_swift_notification_port_mach_port(self.as_ptr()) }
    }

    /// Returns the raw `CFRunLoopSourceRef` pointer as a `usize`.
    pub fn run_loop_source_raw(&self) -> usize {
        unsafe { bridge::iokit_swift_notification_port_run_loop_source(self.as_ptr()) as usize }
    }

    /// Wraps `IONotificationPortSetImportanceReceiver`.
    pub fn set_importance_receiver(&self) -> Result<()> {
        io_result(
            unsafe { bridge::iokit_swift_notification_port_set_importance_receiver(self.as_ptr()) },
            "IONotificationPortSetImportanceReceiver",
        )
    }

    pub fn schedule_on(&self, queue: &DispatchQueue) -> Result<()> {
        if unsafe {
            bridge::iokit_swift_notification_port_schedule(self.as_ptr(), queue.as_ptr().cast_mut())
        } {
            Ok(())
        } else {
            Err(IoKitError::InvalidArgument(
                "the notification port is already scheduled on a dispatch queue".to_string(),
            ))
        }
    }

    pub fn add_matching_notification<F>(
        &self,
        notification_type: &str,
        matching: &MatchingDictionary,
        existing: ExistingServices,
        callback: F,
    ) -> Result<PortNotification>
    where
        F: FnMut(Service) + Send + 'static,
    {
        let notification_type = c_string(notification_type)?;
        let dictionary = dictionary_to_cf(&matching.entries)?;
        let notification = self.register(
            Handler::Matching(Box::new(callback)),
            "IOServiceAddMatchingNotification",
            |port, refcon, notifier| unsafe {
                ffi_impl::IOServiceAddMatchingNotification(
                    port,
                    notification_type.as_ptr(),
                    dictionary.into_raw().cast(),
                    Some(matching_trampoline),
                    refcon,
                    notifier,
                )
            },
        )?;
        deliver_services(
            notification.notifier,
            (existing == ExistingServices::Deliver).then(|| notification.context.get()),
        );
        Ok(notification)
    }

    pub fn add_interest_notification<F>(
        &self,
        service: &Service,
        interest_type: &str,
        callback: F,
    ) -> Result<PortNotification>
    where
        F: FnMut(ServiceInterestEvent) + Send + 'static,
    {
        let interest_type = c_string(interest_type)?;
        self.register(
            Handler::Interest(Box::new(callback)),
            "IOServiceAddInterestNotification",
            |port, refcon, notifier| unsafe {
                ffi_impl::IOServiceAddInterestNotification(
                    port,
                    bridge::iokit_swift_service_raw(service.as_ptr()),
                    interest_type.as_ptr(),
                    Some(interest_trampoline),
                    refcon,
                    notifier,
                )
            },
        )
    }

    fn register(
        &self,
        handler: Handler,
        operation: &'static str,
        add: impl FnOnce(ffi_impl::IONotificationPortRef, *mut c_void, *mut u32) -> i32,
    ) -> Result<PortNotification> {
        let port = self.retained()?;
        let context = PortContext::new(Mutex::new(handler));
        let mut notifier = 0;
        io_result(
            add(
                unsafe { bridge::iokit_swift_notification_port_raw(self.as_ptr()) },
                context.as_ptr(),
                &raw mut notifier,
            ),
            operation,
        )?;
        Ok(PortNotification {
            port,
            notifier,
            context,
        })
    }
}

/// Releases the retained notification-port handle on drop.
impl Drop for NotificationPort {
    fn drop(&mut self) {
        unsafe { bridge::iokit_swift_notification_port_release(self.as_ptr()) };
    }
}

pub struct PortNotification {
    port: NotificationPort,
    notifier: u32,
    context: PortContext,
}

impl std::fmt::Debug for PortNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortNotification")
            .field("port", &self.port)
            .field("notifier", &self.notifier)
            .field("active", &self.context.is_active())
            .finish()
    }
}

impl Drop for PortNotification {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe {
            bridge::iokit_swift_notification_port_release_registration(
                self.port.as_ptr(),
                self.notifier,
                self.context.retained_ptr(),
                PortContext::RELEASE,
            );
        }
    }
}

fn deliver_services(iterator: ffi_impl::io_iterator_t, handler: Option<&Mutex<Handler>>) {
    loop {
        let raw = unsafe { ffi_impl::IOIteratorNext(iterator) };
        if raw == 0 {
            return;
        }
        let Some(handler) = handler else {
            unsafe { ffi_impl::IOObjectRelease(raw) };
            continue;
        };
        let Some(service) = Service::from_raw(unsafe { bridge::iokit_swift_wrap_service(raw) })
        else {
            continue;
        };
        let mut handler = handler.lock().unwrap_or_else(PoisonError::into_inner);
        if let Handler::Matching(callback) = &mut *handler {
            catch_user_panic("NotificationPort::add_matching_notification", || {
                callback(service);
            });
        }
    }
}

unsafe extern "C" fn matching_trampoline(refcon: *mut c_void, iterator: ffi_impl::io_iterator_t) {
    unsafe {
        PortContext::with(
            refcon,
            "NotificationPort::add_matching_notification",
            |handler| deliver_services(iterator, Some(handler)),
        )
    };
}

unsafe extern "C" fn interest_trampoline(
    refcon: *mut c_void,
    _service: ffi_impl::io_service_t,
    message_type: u32,
    message_argument: *mut c_void,
) {
    unsafe {
        PortContext::with(
            refcon,
            "NotificationPort::add_interest_notification",
            |handler| {
                let mut handler = handler.lock().unwrap_or_else(PoisonError::into_inner);
                if let Handler::Interest(callback) = &mut *handler {
                    catch_user_panic("NotificationPort::add_interest_notification", || {
                        callback(ServiceInterestEvent {
                            message: IoMessage::from_raw(message_type),
                            message_argument: message_argument as usize,
                        });
                    });
                }
            },
        )
    };
}
