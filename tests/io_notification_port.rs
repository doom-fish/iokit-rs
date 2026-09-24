use iokit::prelude::*;
use iokit::{
    DispatchQoS, ExistingServices, MatchingDictionary, GENERAL_INTEREST, MATCHED_NOTIFICATION,
};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn wait_until(mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    condition()
}

fn root_domain() -> iokit::Result<Service> {
    matching_service("IOPMrootDomain")?.ok_or(IoKitError::UnexpectedNull("IOPMrootDomain"))
}

fn resources() -> MatchingDictionary {
    MatchingDictionary::class("IOResources")
}

fn within_deadline<T: Send + 'static>(body: impl FnOnce() -> T + Send + 'static) -> T {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let _ = sender.send(body());
    });
    receiver
        .recv_timeout(Duration::from_secs(30))
        .expect("the test body finished without deadlocking")
}

#[test]
fn creates_notification_port() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    assert_ne!(port.mach_port(), 0);
    let source = port.run_loop_source_raw();
    assert_ne!(source, 0);
    let type_id = unsafe { iokit::ffi::CFGetTypeID(source as iokit::ffi::CFTypeRef) };
    assert_eq!(type_id, unsafe {
        apple_cf::raw::CFRunLoopSourceGetTypeID()
    });
    assert_eq!(port.run_loop_source_raw(), source);
    Ok(())
}

#[test]
fn ports_and_registrations_move_across_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NotificationPort>();
    assert_send_sync::<PortNotification>();
}

#[test]
fn a_port_is_scheduled_at_most_once() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    let queue = DispatchQueue::new("fish.doom.iokit.tests.schedule", DispatchQoS::Default);
    port.schedule_on(&queue)?;
    assert!(matches!(
        port.schedule_on(&queue),
        Err(IoKitError::InvalidArgument(_))
    ));
    let other = DispatchQueue::new("fish.doom.iokit.tests.reschedule", DispatchQoS::Default);
    assert!(matches!(
        port.schedule_on(&other),
        Err(IoKitError::InvalidArgument(_))
    ));
    Ok(())
}

#[test]
fn matching_notifications_can_deliver_existing_services() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let delivered = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &resources(),
        ExistingServices::Deliver,
        move |service| {
            sink.lock()
                .unwrap()
                .push(service.class_name().expect("class name"));
        },
    )?;
    assert_eq!(*seen.lock().unwrap(), vec!["IOResources".to_string()]);

    let skipped_calls = Arc::new(Mutex::new(0_usize));
    let counter = Arc::clone(&skipped_calls);
    let skipped = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &resources(),
        ExistingServices::Skip,
        move |_| *counter.lock().unwrap() += 1,
    )?;
    assert_eq!(*skipped_calls.lock().unwrap(), 0);
    drop((delivered, skipped));
    Ok(())
}

#[test]
fn registrations_report_invalid_input() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    assert!(matches!(
        port.add_matching_notification(
            "NoSuchNotificationType",
            &resources(),
            ExistingServices::Skip,
            |_| {},
        ),
        Err(IoKitError::IoReturn("IOServiceAddMatchingNotification", _))
    ));
    assert!(matches!(
        port.add_matching_notification(
            "IOServiceMatched\0",
            &resources(),
            ExistingServices::Skip,
            |_| {},
        ),
        Err(IoKitError::InvalidArgument(_))
    ));
    assert!(port
        .add_matching_notification(
            MATCHED_NOTIFICATION,
            &MatchingDictionary::new().with_matching_key("bad", iokit::CFValue::Unknown(1)),
            ExistingServices::Skip,
            |_| {},
        )
        .is_err());
    assert!(matches!(
        port.add_interest_notification(&root_domain()?, "IOGeneralInterest\0", |_| {}),
        Err(IoKitError::InvalidArgument(_))
    ));
    Ok(())
}

#[test]
fn a_panicking_callback_is_contained() -> iokit::Result<()> {
    let port = NotificationPort::new()?;
    let panicking = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &resources(),
        ExistingServices::Deliver,
        |_| panic!("callback panic"),
    )?;
    let calls = Arc::new(Mutex::new(0_usize));
    let counter = Arc::clone(&calls);
    let working = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &resources(),
        ExistingServices::Deliver,
        move |_| *counter.lock().unwrap() += 1,
    )?;
    assert_eq!(*calls.lock().unwrap(), 1);
    drop((panicking, working));
    Ok(())
}

#[test]
fn dropping_a_registration_frees_its_callback_on_an_unscheduled_port() -> iokit::Result<()> {
    let marker = Arc::new(());
    let port = NotificationPort::new()?;
    let held = Arc::clone(&marker);
    let matched = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &resources(),
        ExistingServices::Skip,
        move |_| {
            let _ = &held;
        },
    )?;
    let held = Arc::clone(&marker);
    let interest =
        port.add_interest_notification(&root_domain()?, GENERAL_INTEREST, move |_| {
            let _ = &held;
        })?;
    assert_eq!(Arc::strong_count(&marker), 3);
    drop(matched);
    assert_eq!(Arc::strong_count(&marker), 2);
    drop(interest);
    assert_eq!(Arc::strong_count(&marker), 1);
    drop(port);
    Ok(())
}

#[test]
fn scheduled_registrations_free_their_callbacks_once_the_queue_drains() -> iokit::Result<()> {
    let marker = Arc::new(());
    let port = NotificationPort::new()?;
    let queue = DispatchQueue::new("fish.doom.iokit.tests.drain", DispatchQoS::Default);
    port.schedule_on(&queue)?;
    let delivered = Arc::new(Mutex::new(0_usize));
    let counter = Arc::clone(&delivered);
    let held = Arc::clone(&marker);
    let matched = port.add_matching_notification(
        MATCHED_NOTIFICATION,
        &resources(),
        ExistingServices::Deliver,
        move |_| {
            let _ = &held;
            *counter.lock().unwrap() += 1;
        },
    )?;
    let held = Arc::clone(&marker);
    let interest =
        port.add_interest_notification(&root_domain()?, GENERAL_INTEREST, move |_| {
            let _ = &held;
        })?;
    assert_eq!(*delivered.lock().unwrap(), 1);
    assert_eq!(Arc::strong_count(&marker), 3);
    drop((matched, interest));
    assert!(wait_until(|| Arc::strong_count(&marker) == 1));
    assert_eq!(*delivered.lock().unwrap(), 1);
    drop(port);
    Ok(())
}

#[test]
fn registrations_keep_their_port_alive() -> iokit::Result<()> {
    let marker = Arc::new(());
    let port = NotificationPort::new()?;
    let held = Arc::clone(&marker);
    let interest =
        port.add_interest_notification(&root_domain()?, GENERAL_INTEREST, move |_| {
            let _ = &held;
        })?;
    drop(port);
    assert_eq!(Arc::strong_count(&marker), 2);
    drop(interest);
    assert_eq!(Arc::strong_count(&marker), 1);
    Ok(())
}

#[test]
fn a_callback_may_own_another_registration() -> iokit::Result<()> {
    for scheduled in [false, true] {
        within_deadline(move || -> iokit::Result<()> {
            let marker = Arc::new(());
            let port = NotificationPort::new()?;
            let queue = DispatchQueue::new("fish.doom.iokit.tests.nested", DispatchQoS::Default);
            if scheduled {
                port.schedule_on(&queue)?;
            }
            let held = Arc::clone(&marker);
            let inner =
                port.add_interest_notification(&root_domain()?, GENERAL_INTEREST, move |_| {
                    let _ = &held;
                })?;
            let outer = port.add_matching_notification(
                MATCHED_NOTIFICATION,
                &resources(),
                ExistingServices::Skip,
                move |_| {
                    let _ = &inner;
                },
            )?;
            drop(port);
            assert_eq!(Arc::strong_count(&marker), 2);
            drop(outer);
            assert!(wait_until(|| Arc::strong_count(&marker) == 1));
            Ok(())
        })?;
    }
    Ok(())
}

#[test]
fn scheduled_ports_register_and_drop_from_any_thread() -> iokit::Result<()> {
    let marker = Arc::new(());
    let service = root_domain()?;
    let port = Arc::new(NotificationPort::new()?);
    let queue = DispatchQueue::new("fish.doom.iokit.tests.port", DispatchQoS::Default);
    port.schedule_on(&queue)?;

    let registrations = thread::scope(|scope| {
        let mut workers = Vec::new();
        for worker in 0..4 {
            let port = Arc::clone(&port);
            let service = service.clone();
            let marker = Arc::clone(&marker);
            workers.push(scope.spawn(move || {
                let mut kept = Vec::new();
                for round in 0..20 {
                    let held = Arc::clone(&marker);
                    let matched = port
                        .add_matching_notification(
                            MATCHED_NOTIFICATION,
                            &resources(),
                            ExistingServices::Deliver,
                            move |service| {
                                let _ = (&held, service.class_name());
                            },
                        )
                        .expect("matching notification");
                    let held = Arc::clone(&marker);
                    let interest = port
                        .add_interest_notification(&service, GENERAL_INTEREST, move |event| {
                            let _ = (&held, event.message);
                        })
                        .expect("interest notification");
                    if (worker + round) % 3 == 0 {
                        kept.push(matched);
                        drop(interest);
                    } else {
                        drop(matched);
                        kept.push(interest);
                    }
                }
                kept
            }));
        }
        let mut registrations = Vec::new();
        for worker in workers {
            registrations.extend(worker.join().expect("worker"));
        }
        registrations
    });
    assert_eq!(registrations.len(), 80);
    assert!(Arc::strong_count(&marker) > 80);

    thread::spawn(move || drop(registrations))
        .join()
        .expect("drop registrations");
    thread::spawn(move || drop(port)).join().expect("drop port");
    drop(queue);
    assert!(wait_until(|| Arc::strong_count(&marker) == 1));
    Ok(())
}
