//! Async Stream API for IOKit (Tier 2 — Stream pattern).
//!
//! Provides executor-agnostic [`doom_fish_utils::stream::BoundedAsyncStream`]
//! wrappers around four IOKit callback surfaces:
//!
//! | Stream type | Underlying API | Events |
//! |---|---|---|
//! | [`ServiceInterestStream`] | `IOServiceAddInterestNotification` | per-service power/state messages |
//! | [`ServiceMatchStream`] | `IOServiceAddMatchingNotification` | service match / terminate |
//! | [`PowerSourceStream`] | `IOPSNotificationCreateRunLoopSource` | power-source changed |
//! | [`SystemPowerStream`] | `IORegisterForSystemPower` | system sleep / wake / shutdown |
//!
//! # Feature gate
//!
//! All types in this module are gated behind the **`async`** Cargo feature:
//! ```toml
//! iokit = { version = "0.3", features = ["async"] }
//! ```
//!
//! # Back-pressure / drop semantics
//!
//! Every stream uses a bounded ring buffer: when full, the **oldest** item is
//! dropped (lossy-latest policy).  A larger `capacity` argument reduces the
//! chance of drops for bursty sources.
//!
//! # Drop / unsubscribe
//!
//! Dropping the `*Stream` struct calls the Swift `_unsubscribe` function, which
//!
//! 1. Stops the IOKit notification mechanism (IOObjectRelease / CFRunLoopRemoveSource).
//! 2. Drains any in-flight callbacks so they cannot touch the freed sender.
//! 3. Releases the Swift bridge object.
//!
//! Immediately after `unsubscribe` returns, Rust drops the boxed sender,
//! which closes the stream for the consumer.
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "async")]
//! # fn main() {
//! use iokit::async_api::{PowerSourceStream};
//!
//! let stream = PowerSourceStream::subscribe(32).expect("subscribe");
//! pollster::block_on(async {
//!     if let Some(_ev) = stream.next().await {
//!         println!("power source changed");
//!     }
//! });
//! # }
//! # #[cfg(not(feature = "async"))] fn main() {}
//! ```

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::non_send_fields_in_send_ty,
    clippy::ptr_cast_constness
)]

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};
use std::ffi::c_void;

use crate::{bridge, error::Result, io_message::IoMessage, io_service::Service, object::c_string, IoKitError};

// ────────────────────────────────────────────────────────────────────────────
// 1. ServiceInterestStream
// ────────────────────────────────────────────────────────────────────────────

/// An IOKit service-interest event produced by [`ServiceInterestStream`].
#[derive(Debug, Clone)]
pub struct ServiceInterestEvent {
    /// Decoded IOKit message type (e.g. `IoMessage::ServiceBusyStateChange`).
    pub message: IoMessage,
    /// Raw message-argument pointer value (may be 0 / null).
    /// Interpretation is message-type–specific; see IOKit documentation.
    pub message_argument: usize,
}

struct ServiceInterestHandle {
    bridge_ptr: *mut c_void,
    sender_ptr: *mut AsyncStreamSender<ServiceInterestEvent>,
}
unsafe impl Send for ServiceInterestHandle {}
unsafe impl Sync for ServiceInterestHandle {}

impl Drop for ServiceInterestHandle {
    fn drop(&mut self) {
        unsafe {
            // 1. Stop callbacks and drain in-flight work.
            bridge::iokit_swift_service_interest_unsubscribe(self.bridge_ptr);
            // 2. Free the sender; closing the stream for consumers.
            drop(Box::from_raw(self.sender_ptr));
        }
    }
}

/// Async stream of [`ServiceInterestEvent`]s for a specific IOKit service.
///
/// Created via [`ServiceInterestStream::subscribe`].  Dropping this value
/// deregisters the notification and closes the stream.
pub struct ServiceInterestStream {
    inner: BoundedAsyncStream<ServiceInterestEvent>,
    _handle: ServiceInterestHandle,
}

unsafe extern "C" fn service_interest_cb(
    kind: i32,
    payload: *const c_void,
    ctx: *mut c_void,
) {
    let sender = unsafe { &*(ctx.cast::<AsyncStreamSender<ServiceInterestEvent>>()) };
    sender.push(ServiceInterestEvent {
        message: IoMessage::from_raw(kind as u32),
        message_argument: payload as usize,
    });
}

impl ServiceInterestStream {
    /// Subscribe to `service` for `interest` notifications (e.g.
    /// [`crate::GENERAL_INTEREST`], [`crate::BUSY_INTEREST`]).
    ///
    /// `capacity` is the size of the ring-buffer backing the stream.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge fails to register the notification.
    pub fn subscribe(service: &Service, interest: &str, capacity: usize) -> Result<Self> {
        let c_interest = c_string(interest)?;
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            bridge::iokit_swift_service_interest_subscribe(
                service.as_ptr(),
                c_interest.as_ptr(),
                service_interest_cb,
                sender_ptr.cast(),
            )
        };
        if bridge_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(IoKitError::UnexpectedNull("iokit_swift_service_interest_subscribe"));
        }
        Ok(Self {
            inner: stream,
            _handle: ServiceInterestHandle { bridge_ptr, sender_ptr },
        })
    }

    /// Await the next [`ServiceInterestEvent`], returning `None` once the
    /// stream is closed (i.e. this `ServiceInterestStream` was dropped).
    pub fn next(&self) -> NextItem<'_, ServiceInterestEvent> {
        self.inner.next()
    }

    /// Non-blocking pop; returns `None` if the buffer is empty.
    pub fn try_next(&self) -> Option<ServiceInterestEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 2. ServiceMatchStream
// ────────────────────────────────────────────────────────────────────────────

/// Whether a service was matched (appeared) or terminated (disappeared).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServiceMatchKind {
    /// A service matching the subscription criteria appeared in the IOKit registry.
    Matched,
    /// A previously matched service has been terminated.
    Terminated,
}

/// An IOKit service-match event produced by [`ServiceMatchStream`].
///
/// The optional `service` field is a retained reference to the service object.
/// Dropping the `ServiceMatchEvent` releases the service.
pub struct ServiceMatchEvent {
    /// Whether the service was matched or terminated.
    pub kind: ServiceMatchKind,
    /// The involved service (retained).  May be `None` if the service pointer
    /// was null in the callback (should not happen under normal conditions).
    pub service: Option<Service>,
}

// Safety: IOKit service objects use ARC-backed retain/release which is
// thread-safe; IOKit registry operations are also thread-safe.
unsafe impl Send for ServiceMatchEvent {}
unsafe impl Sync for ServiceMatchEvent {}

impl std::fmt::Debug for ServiceMatchEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceMatchEvent")
            .field("kind", &self.kind)
            .field("has_service", &self.service.is_some())
            .finish()
    }
}

struct ServiceMatchHandle {
    bridge_ptr: *mut c_void,
    sender_ptr: *mut AsyncStreamSender<ServiceMatchEvent>,
}
unsafe impl Send for ServiceMatchHandle {}
unsafe impl Sync for ServiceMatchHandle {}

impl Drop for ServiceMatchHandle {
    fn drop(&mut self) {
        unsafe {
            bridge::iokit_swift_service_match_unsubscribe(self.bridge_ptr);
            drop(Box::from_raw(self.sender_ptr));
        }
    }
}

/// Async stream of [`ServiceMatchEvent`]s for services matching a class name.
///
/// Created via [`ServiceMatchStream::subscribe`].  Dropping this value
/// deregisters both the match and terminate notifications.
///
/// Note: the initial set of already-matching services is **not** delivered as
/// events.  Call [`crate::matching_services`] to obtain the current set.
pub struct ServiceMatchStream {
    inner: BoundedAsyncStream<ServiceMatchEvent>,
    _handle: ServiceMatchHandle,
}

unsafe extern "C" fn service_match_cb(
    kind: i32,
    payload: *const c_void,
    ctx: *mut c_void,
) {
    let sender = unsafe { &*(ctx.cast::<AsyncStreamSender<ServiceMatchEvent>>()) };
    let service = if payload.is_null() {
        None
    } else {
        // payload is a retained IOObjectHolder*; Service::from_raw takes ownership.
        #[allow(clippy::cast_ptr_alignment)]
        Service::from_raw(payload as *mut c_void)
    };
    let event_kind = if kind == 0 {
        ServiceMatchKind::Matched
    } else {
        ServiceMatchKind::Terminated
    };
    sender.push(ServiceMatchEvent { kind: event_kind, service });
}

impl ServiceMatchStream {
    /// Subscribe to match and terminate events for services of `class_name`.
    ///
    /// `capacity` is the ring-buffer size.
    ///
    /// # Errors
    ///
    /// Returns an error if the Swift bridge fails to register the notifications.
    pub fn subscribe(class_name: &str, capacity: usize) -> Result<Self> {
        let c_name = c_string(class_name)?;
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            bridge::iokit_swift_service_match_subscribe(
                c_name.as_ptr(),
                service_match_cb,
                sender_ptr.cast(),
            )
        };
        if bridge_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(IoKitError::UnexpectedNull("iokit_swift_service_match_subscribe"));
        }
        Ok(Self {
            inner: stream,
            _handle: ServiceMatchHandle { bridge_ptr, sender_ptr },
        })
    }

    /// Await the next [`ServiceMatchEvent`].
    pub fn next(&self) -> NextItem<'_, ServiceMatchEvent> {
        self.inner.next()
    }

    /// Non-blocking pop; returns `None` if the buffer is empty.
    pub fn try_next(&self) -> Option<ServiceMatchEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 3. PowerSourceStream
// ────────────────────────────────────────────────────────────────────────────

/// A power-source-changed notification produced by [`PowerSourceStream`].
///
/// There is no payload — the event merely signals that something changed.
/// Re-query [`crate::iops::PowerSourcesInfo`] to obtain the new state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PowerSourceEvent;

struct PowerSourceHandle {
    bridge_ptr: *mut c_void,
    sender_ptr: *mut AsyncStreamSender<PowerSourceEvent>,
}
unsafe impl Send for PowerSourceHandle {}
unsafe impl Sync for PowerSourceHandle {}

impl Drop for PowerSourceHandle {
    fn drop(&mut self) {
        unsafe {
            bridge::iokit_swift_power_source_unsubscribe(self.bridge_ptr);
            drop(Box::from_raw(self.sender_ptr));
        }
    }
}

/// Async stream of [`PowerSourceEvent`]s driven by
/// `IOPSNotificationCreateRunLoopSource`.
///
/// Each event indicates that the power-source state has changed (AC/battery
/// transition, charge level, etc.).  Re-query `iops::PowerSourcesInfo` for
/// the actual new state.
pub struct PowerSourceStream {
    inner: BoundedAsyncStream<PowerSourceEvent>,
    _handle: PowerSourceHandle,
}

unsafe extern "C" fn power_source_cb(
    _kind: i32,
    _payload: *const c_void,
    ctx: *mut c_void,
) {
    let sender = unsafe { &*(ctx.cast::<AsyncStreamSender<PowerSourceEvent>>()) };
    sender.push(PowerSourceEvent);
}

impl PowerSourceStream {
    /// Subscribe to power-source-changed notifications.
    ///
    /// `capacity` is the ring-buffer size.
    ///
    /// # Errors
    ///
    /// Returns an error if `IOPSNotificationCreateRunLoopSource` fails.
    pub fn subscribe(capacity: usize) -> Result<Self> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            bridge::iokit_swift_power_source_subscribe(power_source_cb, sender_ptr.cast())
        };
        if bridge_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(IoKitError::UnexpectedNull("iokit_swift_power_source_subscribe"));
        }
        Ok(Self {
            inner: stream,
            _handle: PowerSourceHandle { bridge_ptr, sender_ptr },
        })
    }

    /// Await the next [`PowerSourceEvent`].
    pub fn next(&self) -> NextItem<'_, PowerSourceEvent> {
        self.inner.next()
    }

    /// Non-blocking pop; returns `None` if the buffer is empty.
    pub fn try_next(&self) -> Option<PowerSourceEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// 4. SystemPowerStream
// ────────────────────────────────────────────────────────────────────────────

/// A system power-change message produced by [`SystemPowerStream`].
///
/// This is the same [`IoMessage`] type used by the rest of the crate
/// (e.g. `IoMessage::SystemWillSleep`, `IoMessage::SystemHasPoweredOn`).
///
/// The bridge auto-acknowledges `CanSystemSleep`, `SystemWillSleep`,
/// `SystemWillPowerOff`, and `SystemWillRestart` so the system does not stall
/// waiting for an explicit `IOAllowPowerChange` call.
pub type SystemPowerEvent = IoMessage;

struct SystemPowerHandle {
    bridge_ptr: *mut c_void,
    sender_ptr: *mut AsyncStreamSender<SystemPowerEvent>,
}
unsafe impl Send for SystemPowerHandle {}
unsafe impl Sync for SystemPowerHandle {}

impl Drop for SystemPowerHandle {
    fn drop(&mut self) {
        unsafe {
            bridge::iokit_swift_system_power_unsubscribe(self.bridge_ptr);
            drop(Box::from_raw(self.sender_ptr));
        }
    }
}

/// Async stream of [`SystemPowerEvent`]s driven by `IORegisterForSystemPower`.
///
/// Yields system power change messages such as:
/// - `IoMessage::SystemWillSleep`
/// - `IoMessage::SystemWillPowerOn`
/// - `IoMessage::SystemHasPoweredOn`
/// - `IoMessage::SystemWillRestart`
/// - `IoMessage::SystemWillPowerOff`
/// - `IoMessage::CanSystemSleep`
///
/// # Auto-acknowledgment
///
/// The bridge calls `IOAllowPowerChange` automatically for the messages that
/// require it.  Consumers that want to *deny* a sleep request must use the
/// lower-level IOKit API directly.
pub struct SystemPowerStream {
    inner: BoundedAsyncStream<SystemPowerEvent>,
    _handle: SystemPowerHandle,
}

unsafe extern "C" fn system_power_cb(
    kind: i32,
    _payload: *const c_void,
    ctx: *mut c_void,
) {
    let sender = unsafe { &*(ctx.cast::<AsyncStreamSender<SystemPowerEvent>>()) };
    sender.push(IoMessage::from_raw(kind as u32));
}

impl SystemPowerStream {
    /// Subscribe to system power notifications.
    ///
    /// `capacity` is the ring-buffer size.
    ///
    /// # Errors
    ///
    /// Returns an error if `IORegisterForSystemPower` fails.
    pub fn subscribe(capacity: usize) -> Result<Self> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));
        let bridge_ptr = unsafe {
            bridge::iokit_swift_system_power_subscribe(system_power_cb, sender_ptr.cast())
        };
        if bridge_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(IoKitError::UnexpectedNull("iokit_swift_system_power_subscribe"));
        }
        Ok(Self {
            inner: stream,
            _handle: SystemPowerHandle { bridge_ptr, sender_ptr },
        })
    }

    /// Await the next [`SystemPowerEvent`].
    pub fn next(&self) -> NextItem<'_, SystemPowerEvent> {
        self.inner.next()
    }

    /// Non-blocking pop; returns `None` if the buffer is empty.
    pub fn try_next(&self) -> Option<SystemPowerEvent> {
        self.inner.try_next()
    }

    /// Number of events currently buffered.
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}
