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

typealias StreamEventCallback = @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void
typealias StreamContextHook = @convention(c) (UnsafeMutableRawPointer?) -> Void

private let streamQueueKey = DispatchSpecificKey<UnsafeMutableRawPointer>()

private final class StreamContext {
    let onEvent: StreamEventCallback
    let ctx: UnsafeMutableRawPointer
    let release: StreamContextHook

    init(
        onEvent: @escaping StreamEventCallback,
        ctx: UnsafeMutableRawPointer,
        retain: StreamContextHook,
        release: @escaping StreamContextHook
    ) {
        self.onEvent = onEvent
        self.ctx = ctx
        self.release = release
        retain(ctx)
    }

    deinit {
        release(ctx)
    }

    func deliver(_ kind: Int32, _ payload: UnsafeRawPointer?) {
        onEvent(kind, payload, ctx)
    }
}

private class StreamBridge {
    let context: StreamContext
    let queue: DispatchQueue
    var cancelled = false

    init(label: String, context: StreamContext) {
        self.context = context
        self.queue = DispatchQueue(label: label, qos: .default)
        queue.setSpecific(key: streamQueueKey, value: Unmanaged.passUnretained(self).toOpaque())
    }

    var isOnQueue: Bool {
        DispatchQueue.getSpecific(key: streamQueueKey) == Unmanaged.passUnretained(self).toOpaque()
    }

    func stopDelivery() {}

    func teardownOnQueue() {
        cancelled = true
    }

    static func shutdownAndRelease(_ handle: UnsafeMutableRawPointer) {
        let unmanaged = Unmanaged<StreamBridge>.fromOpaque(handle)
        let bridge = unmanaged.takeUnretainedValue()
        bridge.stopDelivery()
        if bridge.isOnQueue {
            bridge.cancelled = true
            bridge.queue.async {
                bridge.teardownOnQueue()
                unmanaged.release()
            }
            return
        }
        bridge.queue.sync { bridge.teardownOnQueue() }
        bridge.queue.sync {}
        unmanaged.release()
    }
}

// MARK: - 1. Service Interest Stream

/// Bridge for IOServiceAddInterestNotification.
/// Delivers interest events (kIOGeneralInterest, kIOBusyInterest, …) for a
/// specific service as (messageType: Int32, messageArgument: void?) tuples.
private final class ServiceInterestBridge: StreamBridge {
    var notifier: io_object_t = 0
    var notifPort: IONotificationPortRef?

    init(context: StreamContext) {
        super.init(label: "fish.doom.iokit.service-interest", context: context)
    }

    override func teardownOnQueue() {
        super.teardownOnQueue()
        if notifier != 0 {
            IOObjectRelease(notifier)
            notifier = 0
        }
        if let port = notifPort {
            IONotificationPortDestroy(port)
            notifPort = nil
        }
    }
}

private let serviceInterestCallback: IOServiceInterestCallback = { refCon, _, messageType, messageArgument in
    guard let refCon else { return }
    let bridge = Unmanaged<ServiceInterestBridge>.fromOpaque(refCon).takeUnretainedValue()
    guard !bridge.cancelled else { return }
    bridge.context.deliver(Int32(bitPattern: messageType), messageArgument)
}

@_cdecl("iokit_swift_service_interest_subscribe")
public func iokit_swift_service_interest_subscribe(
    _ servicePtr: UnsafeMutableRawPointer?,
    _ interestType: UnsafePointer<CChar>?,
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void
) -> UnsafeMutableRawPointer? {
    guard let holder = ioObjectHolder(servicePtr), let interestType else { return nil }

    let bridge = ServiceInterestBridge(
        context: StreamContext(onEvent: onEvent, ctx: ctx, retain: retainContext, release: releaseContext))
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    let registered: Bool = bridge.queue.sync {
        guard let port = IONotificationPortCreate(0) else { return false }
        bridge.notifPort = port
        IONotificationPortSetDispatchQueue(port, bridge.queue)
        var notifier: io_object_t = 0
        let kr = IOServiceAddInterestNotification(
            port, holder.raw, interestType, serviceInterestCallback, bridgePtr, &notifier)
        bridge.notifier = notifier
        return kr == kIOReturnSuccess
    }
    guard registered else {
        StreamBridge.shutdownAndRelease(bridgePtr)
        return nil
    }
    return bridgePtr
}

@_cdecl("iokit_swift_service_interest_unsubscribe")
public func iokit_swift_service_interest_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    StreamBridge.shutdownAndRelease(handle)
}

// MARK: - 2. Service Match Stream

/// Bridge for IOServiceAddMatchingNotification.
/// Delivers matched (kind=0) and terminated (kind=1) events.
/// Payload for each event is a retained IOObjectHolder* wrapping the service.
private final class ServiceMatchBridge: StreamBridge {
    var matchedNotifier: io_iterator_t = 0
    var terminatedNotifier: io_iterator_t = 0
    var notifPort: IONotificationPortRef?

    init(context: StreamContext) {
        super.init(label: "fish.doom.iokit.service-match", context: context)
    }

    override func teardownOnQueue() {
        super.teardownOnQueue()
        if matchedNotifier != 0 {
            IOObjectRelease(matchedNotifier)
            matchedNotifier = 0
        }
        if terminatedNotifier != 0 {
            IOObjectRelease(terminatedNotifier)
            terminatedNotifier = 0
        }
        if let port = notifPort {
            IONotificationPortDestroy(port)
            notifPort = nil
        }
    }

    func drain(_ iterator: io_iterator_t, kind: Int32, deliver: Bool) {
        var service = IOIteratorNext(iterator)
        while service != 0 {
            if deliver && !cancelled {
                context.deliver(kind, UnsafeRawPointer(retainOpaque(IOObjectHolder(service))))
            } else {
                IOObjectRelease(service)
            }
            service = IOIteratorNext(iterator)
        }
    }
}

private let serviceMatchedCallback: IOServiceMatchingCallback = { refCon, iterator in
    guard let refCon else { return }
    Unmanaged<ServiceMatchBridge>.fromOpaque(refCon).takeUnretainedValue()
        .drain(iterator, kind: 0, deliver: true)
}

private let serviceTerminatedCallback: IOServiceMatchingCallback = { refCon, iterator in
    guard let refCon else { return }
    Unmanaged<ServiceMatchBridge>.fromOpaque(refCon).takeUnretainedValue()
        .drain(iterator, kind: 1, deliver: true)
}

@_cdecl("iokit_swift_service_match_subscribe")
public func iokit_swift_service_match_subscribe(
    _ matchingPtr: UnsafeRawPointer?,
    _ deliverExisting: Bool,
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void
) -> UnsafeMutableRawPointer? {
    guard let matchingPtr else { return nil }
    let matching = Unmanaged<CFDictionary>.fromOpaque(matchingPtr).takeRetainedValue()

    let bridge = ServiceMatchBridge(
        context: StreamContext(onEvent: onEvent, ctx: ctx, retain: retainContext, release: releaseContext))
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    let registered: Bool = bridge.queue.sync {
        guard let port = IONotificationPortCreate(0) else { return false }
        bridge.notifPort = port
        IONotificationPortSetDispatchQueue(port, bridge.queue)

        var matchedNotifier: io_iterator_t = 0
        let matchedStatus = IOServiceAddMatchingNotification(
            port, kIOMatchedNotification, matching, serviceMatchedCallback, bridgePtr, &matchedNotifier)
        bridge.matchedNotifier = matchedNotifier
        guard matchedStatus == kIOReturnSuccess else { return false }
        bridge.drain(matchedNotifier, kind: 0, deliver: deliverExisting)

        var terminatedNotifier: io_iterator_t = 0
        let terminatedStatus = IOServiceAddMatchingNotification(
            port, kIOTerminatedNotification, matching, serviceTerminatedCallback, bridgePtr,
            &terminatedNotifier)
        bridge.terminatedNotifier = terminatedNotifier
        guard terminatedStatus == kIOReturnSuccess else { return false }
        bridge.drain(terminatedNotifier, kind: 1, deliver: false)
        return true
    }
    guard registered else {
        StreamBridge.shutdownAndRelease(bridgePtr)
        return nil
    }
    return bridgePtr
}

@_cdecl("iokit_swift_service_match_unsubscribe")
public func iokit_swift_service_match_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    StreamBridge.shutdownAndRelease(handle)
}

// MARK: - 3. Power Source Stream

/// Bridge for IOPSNotificationCreateRunLoopSource.
/// Fires (kind=0, payload=nil) whenever the power-source state changes.
/// Caller should re-query `iops::PowerSourcesInfo` for the new state.
private final class PowerSourceBridge: StreamBridge {
    var source: CFRunLoopSource?

    init(context: StreamContext) {
        super.init(label: "fish.doom.iokit.power-source", context: context)
    }

    override func stopDelivery() {
        removeSharedRunLoopSource(&source)
    }
}

private func removeSharedRunLoopSource(_ source: inout CFRunLoopSource?) {
    guard let src = source else { return }
    CFRunLoopRemoveSource(sharedStreamRunLoop, src, .commonModes)
    source = nil
    // Wait for the shared run loop to process the source removal, flushing
    // any in-flight callback that was already scheduled.
    let sema = DispatchSemaphore(value: 0)
    CFRunLoopPerformBlock(sharedStreamRunLoop, CFRunLoopMode.commonModes.rawValue as CFString) {
        sema.signal()
    }
    CFRunLoopWakeUp(sharedStreamRunLoop)
    sema.wait()
}

@_cdecl("iokit_swift_power_source_subscribe")
public func iokit_swift_power_source_subscribe(
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void
) -> UnsafeMutableRawPointer? {
    let bridge = PowerSourceBridge(
        context: StreamContext(onEvent: onEvent, ctx: ctx, retain: retainContext, release: releaseContext))
    let bridgePtr = Unmanaged.passRetained(bridge).toOpaque()

    let cb: IOPowerSourceCallbackType = { refCon in
        guard let refCon else { return }
        let b = Unmanaged<PowerSourceBridge>.fromOpaque(refCon).takeUnretainedValue()
        b.queue.async {
            guard !b.cancelled else { return }
            b.context.deliver(0, nil)
        }
    }

    guard let src = IOPSNotificationCreateRunLoopSource(cb, bridgePtr)?.takeRetainedValue() else {
        StreamBridge.shutdownAndRelease(bridgePtr)
        return nil
    }
    bridge.source = src
    CFRunLoopAddSource(sharedStreamRunLoop, src, .commonModes)
    CFRunLoopWakeUp(sharedStreamRunLoop)
    return bridgePtr
}

@_cdecl("iokit_swift_power_source_unsubscribe")
public func iokit_swift_power_source_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    StreamBridge.shutdownAndRelease(handle)
}

// MARK: - 4. System Power Stream

/// Bridge for IORegisterForSystemPower.
/// Delivers system power change messages (sleep, wake, shutdown, restart, …)
/// as (messageType: Int32, messageArgument: void?) tuples.
///
/// The bridge auto-acknowledges messages that require it (CanSystemSleep,
/// SystemWillSleep, SystemWillPowerOff, SystemWillRestart) so the system
/// does not stall.
private final class SystemPowerBridge: StreamBridge {
    var connection: io_connect_t = 0
    var notifPort: IONotificationPortRef?
    var notifier: io_object_t = 0
    var source: CFRunLoopSource?

    init(context: StreamContext) {
        super.init(label: "fish.doom.iokit.system-power", context: context)
    }

    override func stopDelivery() {
        removeSharedRunLoopSource(&source)
    }

    deinit {
        if notifier != 0 { IODeregisterForSystemPower(&notifier) }
        if connection != 0 { IOServiceClose(connection) }
        if let port = notifPort { IONotificationPortDestroy(port) }
    }
}

@_cdecl("iokit_swift_system_power_subscribe")
public func iokit_swift_system_power_subscribe(
    _ onEvent: @convention(c) (Int32, UnsafeRawPointer?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer,
    _ retainContext: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ releaseContext: @convention(c) (UnsafeMutableRawPointer?) -> Void
) -> UnsafeMutableRawPointer? {
    let bridge = SystemPowerBridge(
        context: StreamContext(onEvent: onEvent, ctx: ctx, retain: retainContext, release: releaseContext))
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
            b.context.deliver(Int32(bitPattern: mt), ma)
        }
    }

    var notifPortRef: IONotificationPortRef?
    var notifier: io_object_t = 0
    bridge.connection = IORegisterForSystemPower(bridgePtr, &notifPortRef, cb, &notifier)
    bridge.notifier = notifier
    guard bridge.connection != 0, let port = notifPortRef else {
        StreamBridge.shutdownAndRelease(bridgePtr)
        return nil
    }
    bridge.notifPort = port

    guard let src = IONotificationPortGetRunLoopSource(port)?.takeUnretainedValue() else {
        StreamBridge.shutdownAndRelease(bridgePtr)
        return nil
    }
    bridge.source = src
    CFRunLoopAddSource(sharedStreamRunLoop, src, .commonModes)
    CFRunLoopWakeUp(sharedStreamRunLoop)
    return bridgePtr
}

@_cdecl("iokit_swift_system_power_unsubscribe")
public func iokit_swift_system_power_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    StreamBridge.shutdownAndRelease(handle)
}
