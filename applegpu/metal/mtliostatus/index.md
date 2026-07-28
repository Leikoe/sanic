# MTLIOStatus

*Enumeration · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliostatus>

Represents the state of an input/output command buffer.

## Declaration

```swift
enum MTLIOStatus
```

## Topics

### I/O command queue states
- [MTLIOStatus.pending](https://developer.apple.com/documentation/metal/mtliostatus/pending) — Indicates the GPU hasn’t finished executing the input/output command buffer.
- [MTLIOStatus.complete](https://developer.apple.com/documentation/metal/mtliostatus/complete) — Indicates the GPU has successfully finished executing the input/output command buffer.
- [MTLIOStatus.cancelled](https://developer.apple.com/documentation/metal/mtliostatus/cancelled) — Indicates the GPU has successfully abandoned the input/output command buffer.
- [MTLIOStatus.error](https://developer.apple.com/documentation/metal/mtliostatus/error) — Indicates the GPU experienced a problem with the input/output command buffer.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtliostatus/init(rawvalue:))

## See also

### I/O command buffers
- [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) — A command buffer that contains input/output commands that work with files in the file systems and Metal resources.
- [MTLIOFileHandle](https://developer.apple.com/documentation/metal/mtliofilehandle) — Represents a raw or compressed file, such as a resource asset file in your app’s bundle.
- [MTLIOCommandBufferHandler](https://developer.apple.com/documentation/metal/mtliocommandbufferhandler) — A convenience type that defines the signature of an input/output command buffer’s completion handler.
- [MTLIOError.Code](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct/code) — The error codes for creating an input/output file handle.
- [MTLIOErrorDomain](https://developer.apple.com/documentation/metal/mtlioerrordomain) — The domain for input/output command queue errors.
