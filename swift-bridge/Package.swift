// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "IOKitBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "IOKitBridge",
            type: .static,
            targets: ["IOKitBridge"])
    ],
    targets: [
        .target(
            name: "IOKitBridge",
            path: "Sources/IOKitBridge",
            publicHeadersPath: "include")
    ]
)
