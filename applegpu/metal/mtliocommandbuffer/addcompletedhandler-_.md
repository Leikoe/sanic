# addCompletedHandler(_:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer/addcompletedhandler(_:)>

Adds a closure that Metal calls immediately after the GPU finishes executing the commands in the input/output command buffer.

## Declaration

```swift
func addCompletedHandler(_ block: @escaping MTLIOCommandBufferHandler)
```

## Parameters

- **block** — A Swift closure or an Objective-C block with your code.

## See also

### Adding final commands
- [copyStatus(buffer:offset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/copystatus(buffer:offset:)) — Encodes a command that writes the input/output command buffer’s status to a buffer.
