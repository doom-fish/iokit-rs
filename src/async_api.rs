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
//! # `SystemPowerStream` is observation-only
//!
//! The bridge auto-acknowledges sleep/shutdown messages (`CanSystemSleep`,
//! `SystemWillSleep`, `SystemWillPowerOff`, `SystemWillRestart`)
//! **synchronously inside the IOKit callback**, *before* the event is placed
//! in the stream.  Rust code therefore cannot delay or deny a sleep transition
//! via this stream.  Use [`crate::PowerAssertion`] to hold off sleep
//! proactively, or call `IORegisterForSystemPower` directly if you need to
//! deny `CanSystemSleep`.
//!
//! # Feature gate
//!
//! All types in this module are gated behind the **`async`** Cargo feature:
//! ```toml
//! iokit = { version = "0.6", features = ["async"] }
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
//! Dropping the `*Stream` struct deactivates its callback context and then
//! calls the Swift `_unsubscribe` function, which
//!
//! 1. Stops the IOKit notification mechanism (IOObjectRelease / CFRunLoopRemoveSource).
//! 2. Drains any in-flight callbacks on the bridge's private serial queue.
//! 3. Releases the Swift bridge object, which releases its reference to the
//!    callback context.
//!
//! The sender inside the context is dropped, closing the stream for the
//! consumer, once both the bridge and the Rust handle have released it.
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

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};
use std::ffi::c_void;

use crate::{
    bridge,
    cf::dictionary_to_cf,
    error::Result,
    io_message::IoMessage,
    io_service::{MatchingDictionary, Service},
    object::c_string,
    IoKitError,
};

type StreamContext<T> = CallbackContext<AsyncStreamSender<T>>;
type ContextHook = unsafe extern "C" fn(*mut c_void);

struct StreamHandle<T: Send + 'static> {
    bridge_ptr: *mut c_void,
    unsubscribe: unsafe extern "C" fn(*mut c_void),
    context: StreamContext<T>,
}

unsafe impl<T: Send + 'static> Send for StreamHandle<T> {}
unsafe impl<T: Send + 'static> Sync for StreamHandle<T> {}

impl<T: Send + 'static> Drop for StreamHandle<T> {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { (self.unsubscribe)(self.bridge_ptr) };
    }
}

fn subscribe_stream<T: Send + 'static>(
    capacity: usize,
    what: &'static str,
    unsubscribe: unsafe extern "C" fn(*mut c_void),
    subscribe: impl FnOnce(*mut c_void, ContextHook, ContextHook) -> *mut c_void,
) -> Result<(BoundedAsyncStream<T>, StreamHandle<T>)> {
    if capacity == 0 {
        return Err(IoKitError::InvalidArgument(
            "stream capacity must be at least 1".to_string(),
        ));
    }
    let (stream, sender) = BoundedAsyncStream::new(capacity);
    let context = StreamContext::new(sender);
    let bridge_ptr = subscribe(
        context.as_ptr(),
        StreamContext::<T>::RETAIN,
        StreamContext::<T>::RELEASE,
    );
    if bridge_ptr.is_null() {
        return Err(IoKitError::UnexpectedNull(what));
    }
    Ok((
        stream,
        StreamHandle {
            bridge_ptr,
            unsubscribe,
            context,
        },
    ))
}

unsafe fn push_event<T: Send + 'static>(ctx: *mut c_void, site: &str, event: T) {
    unsafe { StreamContext::<T>::with(ctx, site, move |sender| sender.push(event)) };
}

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

/// Async stream of [`ServiceInterestEvent`]s for a specific IOKit service.
///
/// Created via [`ServiceInterestStream::subscribe`].  Dropping this value
/// deregisters the notification and closes the stream.
pub struct ServiceInterestStream {
    inner: BoundedAsyncStream<ServiceInterestEvent>,
    _handle: StreamHandle<ServiceInterestEvent>,
}

unsafe extern "C" fn service_interest_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let event = ServiceInterestEvent {
        message: IoMessage::from_raw(kind as u32),
        message_argument: payload as usize,
    };
    unsafe { push_event(ctx, "service_interest_cb", event) };
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
        let (inner, handle) = subscribe_stream(
            capacity,
            "iokit_swift_service_interest_subscribe",
            bridge::iokit_swift_service_interest_unsubscribe,
            |ctx, retain, release| unsafe {
                bridge::iokit_swift_service_interest_subscribe(
                    service.as_ptr(),
                    c_interest.as_ptr(),
                    service_interest_cb,
                    ctx,
                    retain,
                    release,
                )
            },
        )?;
        Ok(Self {
            inner,
            _handle: handle,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExistingServices {
    #[default]
    Skip,
    Deliver,
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

/// Formats `ServiceMatchEvent` for debugging without forcing a service snapshot.
impl std::fmt::Debug for ServiceMatchEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceMatchEvent")
            .field("kind", &self.kind)
            .field("has_service", &self.service.is_some())
            .finish()
    }
}

/// Async stream of [`ServiceMatchEvent`]s for services matching a class name
/// or a [`MatchingDictionary`].
///
/// Created via [`ServiceMatchStream::subscribe`] or
/// [`ServiceMatchStream::subscribe_matching`].  Dropping this value
/// deregisters both the match and terminate notifications.
///
/// Note: [`ServiceMatchStream::subscribe`] does **not** deliver the initial set
/// of already-matching services as events; pass [`ExistingServices::Deliver`]
/// to [`ServiceMatchStream::subscribe_matching`] to receive them as `Matched`
/// events first, or call [`crate::matching_services`] to obtain the current set.
pub struct ServiceMatchStream {
    inner: BoundedAsyncStream<ServiceMatchEvent>,
    _handle: StreamHandle<ServiceMatchEvent>,
}

unsafe extern "C" fn service_match_cb(kind: i32, payload: *const c_void, ctx: *mut c_void) {
    let event = ServiceMatchEvent {
        kind: if kind == 0 {
            ServiceMatchKind::Matched
        } else {
            ServiceMatchKind::Terminated
        },
        service: Service::from_raw(payload.cast_mut()),
    };
    unsafe { push_event(ctx, "service_match_cb", event) };
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
        Self::subscribe_matching(
            &MatchingDictionary::class(class_name),
            ExistingServices::Skip,
            capacity,
        )
    }

    pub fn subscribe_matching(
        matching: &MatchingDictionary,
        existing: ExistingServices,
        capacity: usize,
    ) -> Result<Self> {
        let dictionary = dictionary_to_cf(&matching.entries)?;
        let (inner, handle) = subscribe_stream(
            capacity,
            "iokit_swift_service_match_subscribe",
            bridge::iokit_swift_service_match_unsubscribe,
            move |ctx, retain, release| unsafe {
                bridge::iokit_swift_service_match_subscribe(
                    dictionary.into_raw(),
                    existing == ExistingServices::Deliver,
                    service_match_cb,
                    ctx,
                    retain,
                    release,
                )
            },
        )?;
        Ok(Self {
            inner,
            _handle: handle,
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

/// Async stream of [`PowerSourceEvent`]s driven by
/// `IOPSNotificationCreateRunLoopSource`.
///
/// Each event indicates that the power-source state has changed (AC/battery
/// transition, charge level, etc.).  Re-query `iops::PowerSourcesInfo` for
/// the actual new state.
pub struct PowerSourceStream {
    inner: BoundedAsyncStream<PowerSourceEvent>,
    _handle: StreamHandle<PowerSourceEvent>,
}

unsafe extern "C" fn power_source_cb(_kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    unsafe { push_event(ctx, "power_source_cb", PowerSourceEvent) };
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
        let (inner, handle) = subscribe_stream(
            capacity,
            "iokit_swift_power_source_subscribe",
            bridge::iokit_swift_power_source_unsubscribe,
            |ctx, retain, release| unsafe {
                bridge::iokit_swift_power_source_subscribe(power_source_cb, ctx, retain, release)
            },
        )?;
        Ok(Self {
            inner,
            _handle: handle,
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
/// **Note:** the bridge auto-acknowledges `CanSystemSleep`, `SystemWillSleep`,
/// `SystemWillPowerOff`, and `SystemWillRestart` synchronously *before*
/// delivering the event.  See [`SystemPowerStream`] for implications.
pub type SystemPowerEvent = IoMessage;

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
/// # Auto-acknowledgment — observation only
///
/// The bridge calls `IOAllowPowerChange` **synchronously inside the IOKit
/// callback**, *before* the event is dispatched to the stream.  This means:
///
/// - The system is never blocked waiting for an ack from Rust code.
/// - Rust consumers **cannot delay or deny** a sleep/shutdown transition
///   using this stream.  By the time `next().await` returns an event, the
///   system has already been permitted to proceed.
///
/// If your application needs to delay sleep (e.g. to flush state before the
/// system suspends), use `IOPMAssertionCreateWithName` / [`crate::PowerAssertion`]
/// to hold a `PreventSystemSleep` assertion while the critical work is in
/// progress, rather than trying to block the power transition here.
///
/// If you need to unconditionally *deny* `CanSystemSleep`, you must register
/// with `IORegisterForSystemPower` directly and call `IOCancelPowerChange`
/// before the kernel timeout (~30 s) expires.
pub struct SystemPowerStream {
    inner: BoundedAsyncStream<SystemPowerEvent>,
    _handle: StreamHandle<SystemPowerEvent>,
}

unsafe extern "C" fn system_power_cb(kind: i32, _payload: *const c_void, ctx: *mut c_void) {
    unsafe { push_event(ctx, "system_power_cb", IoMessage::from_raw(kind as u32)) };
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
        let (inner, handle) = subscribe_stream(
            capacity,
            "iokit_swift_system_power_subscribe",
            bridge::iokit_swift_system_power_unsubscribe,
            |ctx, retain, release| unsafe {
                bridge::iokit_swift_system_power_subscribe(system_power_cb, ctx, retain, release)
            },
        )?;
        Ok(Self {
            inner,
            _handle: handle,
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
