import Foundation

private let ioMessageConstants: [UInt32] = [
    3_758_096_896,
    3_758_096_960,
    3_758_097_008,
    3_758_096_680,
    3_758_097_200,
    3_758_096_933,
    3_758_096_944,
    3_758_097_232,
    3_758_096_928,
    3_758_096_912,
    3_758_096_917,
    3_758_096_672,
    3_758_096_641,
    3_758_096_640,
    3_758_096_432,
    3_758_096_416,
    3_758_096_400,
    3_758_096_688,
    3_758_096_656,
    3_758_097_216,
    3_758_097_152,
    3_758_096_981,
    3_758_096_992,
    3_758_097_040,
    3_758_096_976,
    3_758_097_184,
    3_758_097_168,
    3_758_097_024,
]

@_cdecl("iokit_swift_io_message_constant_count")
public func iokit_swift_io_message_constant_count() -> UInt32 {
    UInt32(ioMessageConstants.count)
}

@_cdecl("iokit_swift_io_message_constant")
public func iokit_swift_io_message_constant(_ index: UInt32) -> UInt32 {
    guard Int(index) < ioMessageConstants.count else {
        return 0
    }
    return ioMessageConstants[Int(index)]
}
