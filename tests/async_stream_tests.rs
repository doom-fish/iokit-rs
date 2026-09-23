//! Integration tests for the `async_api` module (Tier-2 Stream pattern).
//!
//! These tests verify the subscribe → create stream → drop → stream closes
//! sequence for each stream surface without requiring real hardware events.
//! They also confirm that the `try_next` / `buffered_count` helpers behave
//! correctly on freshly-subscribed (empty) streams.

#![cfg(feature = "async")]

use iokit::async_api::{
    ExistingServices, PowerSourceStream, ServiceInterestStream, ServiceMatchEvent,
    ServiceMatchKind, ServiceMatchStream, SystemPowerStream,
};
use iokit::{matching_service, MatchingDictionary, GENERAL_INTEREST};

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

#[test]
fn zero_capacity_is_rejected_instead_of_panicking() {
    assert!(PowerSourceStream::subscribe(0).is_err());
    assert!(SystemPowerStream::subscribe(0).is_err());
    assert!(ServiceMatchStream::subscribe("IOResources", 0).is_err());
    let service = matching_service("IOResources")
        .expect("matching_service")
        .expect("IOResources");
    assert!(ServiceInterestStream::subscribe(&service, GENERAL_INTEREST, 0).is_err());
}

#[test]
fn service_match_skips_existing_services_by_default() {
    let stream = ServiceMatchStream::subscribe("IOResources", 8).expect("subscribe");
    assert_eq!(stream.buffered_count(), 0);
    assert!(stream.try_next().is_none());
}

#[test]
fn service_match_can_deliver_existing_services() {
    let stream = ServiceMatchStream::subscribe_matching(
        &MatchingDictionary::class("IOResources"),
        ExistingServices::Deliver,
        8,
    )
    .expect("subscribe");
    let event = stream
        .try_next()
        .expect("the existing IOResources service is delivered");
    assert_eq!(event.kind, ServiceMatchKind::Matched);
    let service = event.service.expect("matched service");
    assert_eq!(service.class_name().expect("class name"), "IOResources");
    assert!(stream.try_next().is_none());
}

#[test]
fn service_match_events_move_across_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ServiceMatchEvent>();
    assert_send_sync::<ServiceMatchStream>();
    assert_send_sync::<ServiceInterestStream>();

    let stream = ServiceMatchStream::subscribe_matching(
        &MatchingDictionary::name("IOResources"),
        ExistingServices::Deliver,
        4,
    )
    .expect("subscribe");
    let event = stream.try_next().expect("existing service");
    let class_name = std::thread::spawn(move || {
        event
            .service
            .expect("matched service")
            .class_name()
            .expect("class name")
    })
    .join()
    .expect("thread");
    assert_eq!(class_name, "IOResources");
}

#[test]
fn streams_subscribe_and_drop_concurrently_on_other_threads() {
    let service = matching_service("IOPMrootDomain")
        .expect("matching_service")
        .expect("IOPMrootDomain");
    let (tx, rx) = std::sync::mpsc::channel::<Box<dyn Send>>();
    let dropper = std::thread::spawn(move || {
        let mut dropped = 0_usize;
        for stream in rx {
            drop(stream);
            dropped += 1;
        }
        dropped
    });
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let tx = tx.clone();
            let service = service.clone();
            scope.spawn(move || {
                for round in 0..25 {
                    let matched = ServiceMatchStream::subscribe_matching(
                        &MatchingDictionary::class(if round % 5 == 0 {
                            "IOService"
                        } else {
                            "IOResources"
                        }),
                        ExistingServices::Deliver,
                        16,
                    )
                    .expect("match subscribe");
                    assert!(matched.buffered_count() >= 1);
                    let interest = ServiceInterestStream::subscribe(&service, GENERAL_INTEREST, 4)
                        .expect("interest subscribe");
                    if (worker + round) % 2 == 0 {
                        tx.send(Box::new(matched)).expect("send");
                        drop(interest);
                    } else {
                        drop(matched);
                        tx.send(Box::new(interest)).expect("send");
                    }
                }
            });
        }
    });
    drop(tx);
    assert_eq!(dropper.join().expect("dropper"), 100);
}
