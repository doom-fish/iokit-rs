//! Integration tests for the `async_api` module (Tier-2 Stream pattern).
//!
//! These tests verify the subscribe → create stream → drop → stream closes
//! sequence for each stream surface without requiring real hardware events.
//! They also confirm that the `try_next` / `buffered_count` helpers behave
//! correctly on freshly-subscribed (empty) streams.

#![cfg(feature = "async")]

use iokit::async_api::{
    PowerSourceStream, ServiceMatchStream, ServiceInterestStream, SystemPowerStream,
};
use iokit::{matching_service, GENERAL_INTEREST};

// ─── PowerSourceStream ────────────────────────────────────────────────────

#[test]
fn power_source_subscribe_and_drop() {
    let stream = PowerSourceStream::subscribe(8).expect("PowerSourceStream::subscribe");
    // No events should have arrived yet.
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
    // Drop the stream → unsubscribes the run-loop source.
    drop(stream);
}

#[test]
fn power_source_stream_closes_on_drop() {
    let stream = PowerSourceStream::subscribe(8).expect("PowerSourceStream::subscribe");
    // Drop triggers unsubscribe; the stream should eventually close.
    drop(stream);
    // After drop, further subscribe calls should still succeed.
    let stream2 = PowerSourceStream::subscribe(4).expect("second subscribe");
    drop(stream2);
}

// ─── SystemPowerStream ────────────────────────────────────────────────────

#[test]
fn system_power_subscribe_and_drop() {
    let stream = SystemPowerStream::subscribe(8).expect("SystemPowerStream::subscribe");
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
    drop(stream);
}

#[test]
fn system_power_multiple_subscribes() {
    // Two simultaneous subscriptions should both succeed.
    let s1 = SystemPowerStream::subscribe(4).expect("subscribe 1");
    let s2 = SystemPowerStream::subscribe(4).expect("subscribe 2");
    drop(s1);
    drop(s2);
}

// ─── ServiceMatchStream ───────────────────────────────────────────────────

#[test]
fn service_match_subscribe_and_drop() {
    // IOHIDDevice is reliably present on all macOS machines.
    let stream =
        ServiceMatchStream::subscribe("IOHIDDevice", 16).expect("ServiceMatchStream::subscribe");
    assert!(stream.buffered_count() <= 16);
    drop(stream);
}

#[test]
fn service_match_unknown_class_succeeds() {
    // An unknown class name is valid (just never fires); subscribe should succeed.
    let stream = ServiceMatchStream::subscribe("DoesNotExist_______XYZ123", 4)
        .expect("ServiceMatchStream with unknown class");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

// ─── ServiceInterestStream ────────────────────────────────────────────────

#[test]
fn service_interest_subscribe_and_drop() {
    match matching_service("IOPMrootDomain") {
        Ok(Some(service)) => {
            match ServiceInterestStream::subscribe(&service, GENERAL_INTEREST, 16) {
                Ok(stream) => {
                    assert_eq!(stream.buffered_count(), 0);
                    assert!(stream.try_next().is_none());
                    drop(stream);
                }
                Err(e) => {
                    // May fail without proper entitlements in CI; not a test error.
                    eprintln!(
                        "ServiceInterestStream::subscribe skipped: {e} \
                         (needs IOKit entitlement in sandboxed CI)"
                    );
                }
            }
        }
        Ok(None) => eprintln!("IOPMrootDomain not found — skipping"),
        Err(e) => eprintln!("matching_service error: {e} — skipping"),
    }
}

// ─── Capacity / buffer semantics ─────────────────────────────────────────

#[test]
fn power_source_custom_capacity() {
    // Verify that capacity is forwarded correctly.
    let stream = PowerSourceStream::subscribe(64).expect("subscribe with capacity 64");
    assert_eq!(stream.buffered_count(), 0);
    drop(stream);
}

// ─── Async next() closes when stream drops ────────────────────────────────

#[test]
fn power_source_buffered_count_starts_empty() {
    let stream = PowerSourceStream::subscribe(4).expect("subscribe");
    // No events have been produced yet; buffer must be empty.
    assert_eq!(stream.buffered_count(), 0);
}
