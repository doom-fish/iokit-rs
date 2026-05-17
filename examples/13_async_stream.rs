//! Example 13 — Async stream API for `IOKit` events.
//!
//! Demonstrates subscribing to each of the four Tier-2 stream surfaces:
//!
//! * `ServiceInterestStream` — interest notifications on a real service
//! * `ServiceMatchStream`    — match / terminate events for `IOHIDDevice`
//! * `PowerSourceStream`     — power-source-changed notifications
//! * `SystemPowerStream`     — system sleep / wake / shutdown events
//!
//! The example uses short timeouts so it exits quickly on headless CI machines
//! without any power-state changes occurring.  On a laptop, plugging/unplugging
//! the charger while this runs will produce a `PowerSourceEvent`.
//!
//! Run with:
//! ```
//! cargo run --example 13_async_stream --features async
//! ```

#[cfg(feature = "async")]
fn main() {
    use iokit::{
        async_api::{PowerSourceStream, ServiceInterestStream, ServiceMatchStream, SystemPowerStream},
        matching_service, GENERAL_INTEREST,
    };
    use std::time::Duration;

    println!("=== IOKit Async Stream example ===");

    // ── 1. ServiceInterestStream ──────────────────────────────────────────
    println!("\n[1] ServiceInterestStream on IOPMrootDomain (GENERAL_INTEREST)");
    match matching_service("IOPMrootDomain") {
        Ok(Some(service)) => {
            match ServiceInterestStream::subscribe(&service, GENERAL_INTEREST, 16) {
                Ok(stream) => {
                    println!("    subscribed — waiting up to 200 ms for an event …");
                    let result = poll_timeout(stream.next(), Duration::from_millis(200));
                    match result {
                        Some(Some(ev)) => println!("    got event: {ev:?}"),
                        Some(None) | None => {
                            println!("    no event within timeout (expected on quiet system)");
                        }
                    }
                }
                Err(e) => println!("    subscribe failed: {e} (may need entitlements)"),
            }
        }
        Ok(None) => println!("    IOPMrootDomain not found"),
        Err(e) => println!("    matching_service error: {e}"),
    }

    // ── 2. ServiceMatchStream ─────────────────────────────────────────────
    println!("\n[2] ServiceMatchStream for IOHIDDevice");
    match ServiceMatchStream::subscribe("IOHIDDevice", 16) {
        Ok(stream) => {
            println!("    subscribed — waiting up to 200 ms …");
            let result = poll_timeout(stream.next(), Duration::from_millis(200));
            match result {
                Some(Some(ev)) => {
                    println!(
                        "    got {:?} event (service present: {})",
                        ev.kind,
                        ev.service.is_some()
                    );
                }
                Some(None) | None => {
                    println!("    no match/terminate event within timeout (expected on quiet system)");
                }
            }
        }
        Err(e) => println!("    subscribe failed: {e}"),
    }

    // ── 3. PowerSourceStream ──────────────────────────────────────────────
    println!("\n[3] PowerSourceStream");
    match PowerSourceStream::subscribe(16) {
        Ok(stream) => {
            println!("    subscribed — waiting up to 200 ms …");
            let result = poll_timeout(stream.next(), Duration::from_millis(200));
            match result {
                Some(Some(_ev)) => println!("    power source changed!"),
                Some(None) | None => {
                    println!(
                        "    no power-source event within timeout (plug/unplug charger to trigger)"
                    );
                }
            }
        }
        Err(e) => println!("    subscribe failed: {e}"),
    }

    // ── 4. SystemPowerStream ──────────────────────────────────────────────
    println!("\n[4] SystemPowerStream");
    match SystemPowerStream::subscribe(16) {
        Ok(stream) => {
            println!("    subscribed — waiting up to 200 ms …");
            let result = poll_timeout(stream.next(), Duration::from_millis(200));
            match result {
                Some(Some(ev)) => println!("    system power event: {ev:?}"),
                Some(None) | None => {
                    println!("    no system-power event within timeout (sleep/wake to trigger)");
                }
            }
        }
        Err(e) => println!("    subscribe failed: {e}"),
    }

    println!("\nDone.");
}

/// Spin-polls `fut` up to `timeout`, sleeping 1 ms between polls.
/// Returns `Some(T)` if the future resolves within the deadline, else `None`.
///
/// This is deliberately minimal — no async executor needed.
#[cfg(feature = "async")]
fn poll_timeout<F: std::future::Future>(fut: F, timeout: std::time::Duration) -> Option<F::Output> {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    use std::time::Instant;

    static NOOP_VTABLE: RawWakerVTable = {
        const fn clone_noop(data: *const ()) -> RawWaker {
            RawWaker::new(data, &NOOP_VTABLE)
        }
        const fn noop(_: *const ()) {}
        RawWakerVTable::new(clone_noop, noop, noop, noop)
    };
    let raw_waker = RawWaker::new(std::ptr::null(), &NOOP_VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut cx = Context::from_waker(&waker);
    let deadline = Instant::now() + timeout;
    let mut pinned = Box::pin(fut);
    loop {
        if let Poll::Ready(val) = pinned.as_mut().poll(&mut cx) {
            return Some(val);
        }
        if Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("Rerun with --features async");
    std::process::exit(1);
}

