# copyStatus(buffer:offset:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer/copystatus(buffer:offset:)>

Encodes a command that writes the input/output command buffer’s status to a buffer.

## Declaration

```swift
func copyStatus(buffer: any MTLBuffer, offset: Int)
```

## Parameters

- **buffer** — A buffer instance the method copies the status into.
- **offset** — A starting location relative to the beginning of the buffer, in bytes, the method copies data to.

## See also

### Adding final commands
- [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/addcompletedhandler(_:)) — Adds a closure that Metal calls immediately after the GPU finishes executing the commands in the input/output command buffer.
