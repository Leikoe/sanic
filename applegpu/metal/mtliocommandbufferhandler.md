# MTLIOCommandBufferHandler

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtliocommandbufferhandler>

A convenience type that defines the signature of an input/output command buffer’s completion handler.

## Declaration

```swift
typealias MTLIOCommandBufferHandler = @Sendable (any MTLIOCommandBuffer) -> Void
```

## Parameters

- **inputOutputCommandBuffer** — The [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) instance that has finished executing is calling your completion handler.

## See also

### I/O command buffers
- [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) — A command buffer that contains input/output commands that work with files in the file systems and Metal resources.
- [MTLIOFileHandle](https://developer.apple.com/documentation/metal/mtliofilehandle) — Represents a raw or compressed file, such as a resource asset file in your app’s bundle.
- [MTLIOStatus](https://developer.apple.com/documentation/metal/mtliostatus) — Represents the state of an input/output command buffer.
- [MTLIOError.Code](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct/code) — The error codes for creating an input/output file handle.
- [MTLIOErrorDomain](https://developer.apple.com/documentation/metal/mtlioerrordomain) — The domain for input/output command queue errors.
