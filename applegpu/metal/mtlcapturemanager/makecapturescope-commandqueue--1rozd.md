# makeCaptureScope(commandQueue:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(commandqueue:)-1rozd>

Creates a capture scope for commands submitted to a specific command queue.

## Declaration

```swift
func makeCaptureScope(commandQueue: any MTLCommandQueue) -> any MTLCaptureScope
```

## Parameters

- **commandQueue** — The command queue whose commands you want to capture.

## See also

### Creating a capture scope
- [makeCaptureScope(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(device:)) — Creates a capture scope for commands submitted to a specific device object.
- [defaultCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturemanager/defaultcapturescope) — The capture scope to use when a capture is initiated in Xcode.
