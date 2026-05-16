import Foundation

@_cdecl("iokit_swift_iohi_backing_store_public_sdk_available")
public func iokit_swift_iohi_backing_store_public_sdk_available() -> Bool {
    false
}

@_cdecl("iokit_swift_iohi_backing_store_unavailability_reason")
public func iokit_swift_iohi_backing_store_unavailability_reason() -> UnsafeMutablePointer<CChar>? {
    strdup("IOHIBackingStore is a private IOKit class and is absent from the public macOS SDK headers.")
}
