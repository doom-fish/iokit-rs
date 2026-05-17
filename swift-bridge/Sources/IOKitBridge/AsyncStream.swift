// AsyncStream.swift — Tier-2 async stream bridges for IOKit callback APIs.
//
// Four stream surfaces:
//  1. ServiceInterestBridge  — IOServiceAddInterestNotification
//  2. ServiceMatchBridge     — IOServiceAddMatchingNotification (match + terminate)
//  3. PowerSourceBridge      — IOPSNotificationCreateRunLoopSource
//  4. SystemPowerBridge      — IORegisterForSystemPower
//
// Dispatch-queue pattern (ServiceInterest, ServiceMatch): callbacks are
// delivered via IONotificationPortSetDispatchQueue on a private serial queue.
// Unsubscribe drains the queue with `queue.sync {}`.
//
// RunLoop pattern (PowerSource, SystemPower): a shared background CFRunLoop
// thread drives the run-loop sources.  Each bridge dispatches its onEvent
// call to an internal serial queue so that unsubscribe can drain with
// `queue.sync {}`.

import CoreFoundation
import Dispatch
import Foundation
import IOKit
import IOKit.ps
import IOKit.pwr_mgt

// MARK: - Shared background CFRunLoop

/// A single background thread whose CFRunLoop drives all IOKit run-loop
/// sources created by this module (PowerSource and SystemPower streams).
private let sharedStreamRunLoop: CFRunLoop = {
    var result: CFRunLoop?
    let sema = DispatchSemaphore(value: 0)
    let thread = Thread {
        result = CFRunLoopGetCurrent()
        // Keepalive: a repeating timer set far in the future so the run loop
        // never exits due to an empty source list.
        var ctx = CFRunLoopTimerContext()
        let keepalive = CFRunLoopTimerCreate(
            nil,
            CFAbsoluteTimeGetCurrent() + 1.0e10, // first fire: ~317 years
            1.0e10,                               // repeat interval
            0, 0,
            { _, _ in },
            &ctx
        )
        CFRunLoopAddTimer(CFRunLoopGetCurrent(), keepalive, .commonModes)
        sema.signal()
        CFRunLoopRun()
    }
    thread.name = "fish.doom.iokit.stream-loop"
    thread.qualityOfService = .default
    thread.start()
    sema.wait()
    return result!
}()

// IOKit system-power message types that require acknowledgment via
// IOAllowPowerChange (values from <IOKit/pwr_mgt/IOPMLib.h>).
private let kMsgCanSystemSleep: UInt32  = 3_758_097_008   // 0xE0000270
private let kMsgSystemWillSleep: UInt32 = 3_758_097_024   // 0xE0000280
private let kMsgWillPowerOff: UInt32    = 3_758_096_976   // 0xE0000250
private let kMsgWillRestart: UInt32     = 3_758_097_168   // 0xE0000310

// MARK: - 1. Service Interest Stream

/// Bridge for IOServiceAddInterestNotification.
/// Delivers interest events (kIOGeneralInterest, kIOBusyInterest, …) for a
/// specific service as (messageType: Int32, messageArgument: void?) tuples.
private final class ServiceInterestBridge {
    let onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void
    let ctx: UnsafeMutableRawPointer
    var notifier: io_object_t = 0
    let notifPort: IONotificationPortRef
    let queue: DispatchQueue
    var cancelled = false   // accessed only on `queue`

    init(
        onEvent: @escaping @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
        ctx: UnsafeMutableRawPointer,
        notifPort: IONotificationPortRef
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.notifPort = notifPort
        self.queue = DispatchQueue(label: "fish.doom.iokit.service-interest", qos: .default)
    }

    deinit {
        if notifier != 0 { IOObjectRelease(notifier) }
        IONotificationPortDestroy(notifPort)
    }
}

@_cdecl("iokit_swift_service_interest_subscribe")
public func iokit_swift_service_interest_subscribe(
    _ servicePtr: UnsafeMutableRawPointer?,
    _ interestType: UnsafePointer<CChar>?,
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let holder = ioObjectHolder(servicePtr), let interestType else { return nil }
    guard let notifPort = IONotificationPortCreate(0) else { return nil }
    IONotificationPortSetDispatchQueue(notifPort, DispatchQueue.global(qos: .default))

    let bridge = ServiceInterestBridge(onEvent: onEvent, ctx: ctx, notifPort: notifPort)
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    let cb: IOServiceInterestCallback = { refCon, _, messageType, messageArgument in
        guard let refCon else { return }
        let b = Unmanaged<ServiceInterestBridge>.fromOpaque(refCon).takeUnretainedValue()
        let mt = messageType
        let ma = messageArgument
        b.queue.async {
            guard !b.cancelled else { return }
            b.onEvent(Int32(bitPattern: mt), ma, b.ctx)
        }
    }

    let kr = IOServiceAddInterestNotification(
        notifPort, holder.raw, interestType, cb, bridgePtr, &bridge.notifier)
    guard kr == kIOReturnSuccess else {
        Unmanaged<ServiceInterestBridge>.fromOpaque(bridgePtr).release()
        return nil
    }
    return bridgePtr
}

@_cdecl("iokit_swift_service_interest_unsubscribe")
public func iokit_swift_service_interest_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let unmanaged = Unmanaged<ServiceInterestBridge>.fromOpaque(handle)
    let b = unmanaged.takeUnretainedValue()
    // Stop IOKit from dispatching new callbacks.
    if b.notifier != 0 { IOObjectRelease(b.notifier); b.notifier = 0 }
    // Drain the serial queue: after this sync returns, no callback can touch ctx.
    b.queue.sync { b.cancelled = true }
    unmanaged.release()
}

// MARK: - 2. Service Match Stream

/// Bridge for IOServiceAddMatchingNotification.
/// Delivers matched (kind=0) and terminated (kind=1) events.
/// Payload for each event is a retained IOObjectHolder* wrapping the service.
private final class ServiceMatchBridge {
    let onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void
    let ctx: UnsafeMutableRawPointer
    var matchedNotifier: io_object_t = 0
    var terminatedNotifier: io_object_t = 0
    let notifPort: IONotificationPortRef
    let queue: DispatchQueue
    var cancelled = false

    init(
        onEvent: @escaping @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
        ctx: UnsafeMutableRawPointer,
        notifPort: IONotificationPortRef
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.notifPort = notifPort
        self.queue = DispatchQueue(label: "fish.doom.iokit.service-match", qos: .default)
    }

    deinit {
        if matchedNotifier != 0 { IOObjectRelease(matchedNotifier) }
        if terminatedNotifier != 0 { IOObjectRelease(terminatedNotifier) }
        IONotificationPortDestroy(notifPort)
    }
}

/// Drain an IOKit iterator, optionally dispatching events.
private func drainIterator(
    _ iterator: io_iterator_t,
    kind: Int32,
    bridge: ServiceMatchBridge,
    sendEvents: Bool
) {
    var service = IOIteratorNext(iterator)
    while service != 0 {
        if sendEvents {
            // IOIteratorNext gives +1 retain; IOObjectHolder takes ownership.
            let holderPtr = retainOpaque(IOObjectHolder(service))
            let q = bridge.queue
            let ctx = bridge.ctx
            let ev = bridge.onEvent
            q.async { [weak bridge] in
                guard let bridge = bridge, !bridge.cancelled else {
                    // Nobody will take ownership; release the holder ourselves.
                    Unmanaged<AnyObject>.fromOpaque(holderPtr).release()
                    return
                }
                ev(kind, holderPtr, ctx)
                // Rust side wraps holderPtr in Service; Service.drop releases it.
            }
        } else {
            IOObjectRelease(service)
        }
        service = IOIteratorNext(iterator)
    }
}

@_cdecl("iokit_swift_service_match_subscribe")
public func iokit_swift_service_match_subscribe(
    _ className: UnsafePointer<CChar>?,
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let className else { return nil }
    guard let notifPort = IONotificationPortCreate(0) else { return nil }
    IONotificationPortSetDispatchQueue(notifPort, DispatchQueue.global(qos: .default))

    let bridge = ServiceMatchBridge(onEvent: onEvent, ctx: ctx, notifPort: notifPort)
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    // --- matched callback ---
    let matchedCb: IOServiceMatchingCallback = { refCon, iterator in
        guard let refCon else { return }
        let b = Unmanaged<ServiceMatchBridge>.fromOpaque(refCon).takeUnretainedValue()
        drainIterator(iterator, kind: 0, bridge: b, sendEvents: true)
    }

    // --- terminated callback ---
    let terminatedCb: IOServiceMatchingCallback = { refCon, iterator in
        guard let refCon else { return }
        let b = Unmanaged<ServiceMatchBridge>.fromOpaque(refCon).takeUnretainedValue()
        drainIterator(iterator, kind: 1, bridge: b, sendEvents: true)
    }

    // Register for matched notifications.
    let kr1 = IOServiceAddMatchingNotification(
        notifPort,
        kIOMatchedNotification,
        IOServiceMatching(className),
        matchedCb,
        bridgePtr,
        &bridge.matchedNotifier
    )
    guard kr1 == kIOReturnSuccess else {
        Unmanaged<ServiceMatchBridge>.fromOpaque(bridgePtr).release()
        return nil
    }
    // Drain initial set (already-matching services) silently to arm the notifier.
    drainIterator(bridge.matchedNotifier, kind: 0, bridge: bridge, sendEvents: false)

    // Register for terminated notifications.
    let kr2 = IOServiceAddMatchingNotification(
        notifPort,
        kIOTerminatedNotification,
        IOServiceMatching(className),
        terminatedCb,
        bridgePtr,
        &bridge.terminatedNotifier
    )
    guard kr2 == kIOReturnSuccess else {
        Unmanaged<ServiceMatchBridge>.fromOpaque(bridgePtr).release()
        return nil
    }
    drainIterator(bridge.terminatedNotifier, kind: 1, bridge: bridge, sendEvents: false)

    return bridgePtr
}

@_cdecl("iokit_swift_service_match_unsubscribe")
public func iokit_swift_service_match_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let unmanaged = Unmanaged<ServiceMatchBridge>.fromOpaque(handle)
    let b = unmanaged.takeUnretainedValue()
    if b.matchedNotifier != 0 { IOObjectRelease(b.matchedNotifier); b.matchedNotifier = 0 }
    if b.terminatedNotifier != 0 { IOObjectRelease(b.terminatedNotifier); b.terminatedNotifier = 0 }
    b.queue.sync { b.cancelled = true }
    unmanaged.release()
}

// MARK: - 3. Power Source Stream

/// Bridge for IOPSNotificationCreateRunLoopSource.
/// Fires (kind=0, payload=nil) whenever the power-source state changes.
/// Caller should re-query `iops::PowerSourcesInfo` for the new state.
private final class PowerSourceBridge {
    let onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void
    let ctx: UnsafeMutableRawPointer
    var source: CFRunLoopSource?
    let queue: DispatchQueue
    var cancelled = false

    init(
        onEvent: @escaping @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
        ctx: UnsafeMutableRawPointer
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.queue = DispatchQueue(label: "fish.doom.iokit.power-source", qos: .default)
    }

    deinit {
        if let src = source {
            CFRunLoopRemoveSource(sharedStreamRunLoop, src, .commonModes)
        }
    }
}

@_cdecl("iokit_swift_power_source_subscribe")
public func iokit_swift_power_source_subscribe(
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let bridge = PowerSourceBridge(onEvent: onEvent, ctx: ctx)
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    let cb: IOPowerSourceCallbackType = { refCon in
        guard let refCon else { return }
        let b = Unmanaged<PowerSourceBridge>.fromOpaque(refCon).takeUnretainedValue()
        b.queue.async {
            guard !b.cancelled else { return }
            b.onEvent(0, nil, b.ctx)
        }
    }

    guard let src = IOPSNotificationCreateRunLoopSource(cb, bridgePtr)?.takeRetainedValue() else {
        Unmanaged<PowerSourceBridge>.fromOpaque(bridgePtr).release()
        return nil
    }
    bridge.source = src
    CFRunLoopAddSource(sharedStreamRunLoop, src, .commonModes)
    CFRunLoopWakeUp(sharedStreamRunLoop)
    return bridgePtr
}

@_cdecl("iokit_swift_power_source_unsubscribe")
public func iokit_swift_power_source_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let unmanaged = Unmanaged<PowerSourceBridge>.fromOpaque(handle)
    let b = unmanaged.takeUnretainedValue()
    if let src = b.source {
        CFRunLoopRemoveSource(sharedStreamRunLoop, src, .commonModes)
        b.source = nil
    }
    // Wait for the run loop to process the removal, then drain the serial queue.
    let sema = DispatchSemaphore(value: 0)
    CFRunLoopPerformBlock(sharedStreamRunLoop, CFRunLoopMode.commonModes.rawValue as CFString) {
        sema.signal()
    }
    CFRunLoopWakeUp(sharedStreamRunLoop)
    sema.wait()
    b.queue.sync { b.cancelled = true }
    unmanaged.release()
}

// MARK: - 4. System Power Stream

/// Bridge for IORegisterForSystemPower.
/// Delivers system power change messages (sleep, wake, shutdown, restart, …)
/// as (messageType: Int32, messageArgument: void?) tuples.
///
/// The bridge auto-acknowledges messages that require it (CanSystemSleep,
/// SystemWillSleep, SystemWillPowerOff, SystemWillRestart) so the system
/// does not stall.
private final class SystemPowerBridge {
    let onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void
    let ctx: UnsafeMutableRawPointer
    var connection: io_connect_t = 0
    var notifPort: IONotificationPortRef?
    var notifier: io_object_t = 0
    var source: CFRunLoopSource?
    let queue: DispatchQueue
    var cancelled = false

    init(
        onEvent: @escaping @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
        ctx: UnsafeMutableRawPointer
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.queue = DispatchQueue(label: "fish.doom.iokit.system-power", qos: .default)
    }

    deinit {
        if let src = source {
            CFRunLoopRemoveSource(sharedStreamRunLoop, src, .commonModes)
        }
        if notifier != 0 { IODeregisterForSystemPower(&notifier) }
        if connection != 0 { IOServiceClose(connection) }
        if let port = notifPort { IONotificationPortDestroy(port) }
    }
}

@_cdecl("iokit_swift_system_power_subscribe")
public func iokit_swift_system_power_subscribe(
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    let bridge = SystemPowerBridge(onEvent: onEvent, ctx: ctx)
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    let cb: IOServiceInterestCallback = { refCon, _, messageType, messageArgument in
        guard let refCon else { return }
        let b = Unmanaged<SystemPowerBridge>.fromOpaque(refCon).takeUnretainedValue()
        let mt = messageType
        let ma = messageArgument
        // Auto-acknowledge messages that require it, synchronously, before
        // dispatching to the user queue.  Delaying ack beyond the kernel
        // timeout (≈30 s for sleep) would cause the system to proceed anyway.
        if mt == kMsgCanSystemSleep || mt == kMsgSystemWillSleep ||
           mt == kMsgWillPowerOff   || mt == kMsgWillRestart {
            IOAllowPowerChange(b.connection, Int(bitPattern: ma))
        }
        b.queue.async {
            guard !b.cancelled else { return }
            b.onEvent(Int32(bitPattern: mt), ma, b.ctx)
        }
    }

    var notifPortRef: IONotificationPortRef?
    bridge.connection = IORegisterForSystemPower(
        bridgePtr, &notifPortRef, cb, &bridge.notifier)
    guard bridge.connection != 0, let port = notifPortRef else {
        Unmanaged<SystemPowerBridge>.fromOpaque(bridgePtr).release()
        return nil
    }
    bridge.notifPort = port

    guard let src = IONotificationPortGetRunLoopSource(port)?.takeUnretainedValue() else {
        Unmanaged<SystemPowerBridge>.fromOpaque(bridgePtr).release()
        return nil
    }
    bridge.source = src
    CFRunLoopAddSource(sharedStreamRunLoop, src, .commonModes)
    CFRunLoopWakeUp(sharedStreamRunLoop)
    return bridgePtr
}

@_cdecl("iokit_swift_system_power_unsubscribe")
public func iokit_swift_system_power_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let unmanaged = Unmanaged<SystemPowerBridge>.fromOpaque(handle)
    let b = unmanaged.takeUnretainedValue()
    if let src = b.source {
        CFRunLoopRemoveSource(sharedStreamRunLoop, src, .commonModes)
        b.source = nil
    }
    // Wait for the shared run loop to process the source removal, flushing
    // any in-flight callback that was already scheduled.
    let sema = DispatchSemaphore(value: 0)
    CFRunLoopPerformBlock(sharedStreamRunLoop, CFRunLoopMode.commonModes.rawValue as CFString) {
        sema.signal()
    }
    CFRunLoopWakeUp(sharedStreamRunLoop)
    sema.wait()
    // Drain the serial queue: after this sync, no callback can touch ctx.
    b.queue.sync { b.cancelled = true }
    unmanaged.release()
}
