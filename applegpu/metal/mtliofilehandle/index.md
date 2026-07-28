# MTLIOFileHandle

*Protocol · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliofilehandle>

Represents a raw or compressed file, such as a resource asset file in your app’s bundle.

## Declaration

```swift
protocol MTLIOFileHandle : NSObjectProtocol, Sendable
```

## Topics

### Naming a file handle
- [label](https://developer.apple.com/documentation/metal/mtliofilehandle/label) — An optional name for the file that the handle represents.

## See also

### I/O command buffers
- [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) — A command buffer that contains input/output commands that work with files in the file systems and Metal resources.
- [MTLIOCommandBufferHandler](https://developer.apple.com/documentation/metal/mtliocommandbufferhandler) — A convenience type that defines the signature of an input/output command buffer’s completion handler.
- [MTLIOStatus](https://developer.apple.com/documentation/metal/mtliostatus) — Represents the state of an input/output command buffer.
- [MTLIOError.Code](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct/code) — The error codes for creating an input/output file handle.
- [MTLIOErrorDomain](https://developer.apple.com/documentation/metal/mtlioerrordomain) — The domain for input/output command queue errors.
