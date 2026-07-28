# makeCaptureScope(device:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(device:)>

Creates a capture scope for commands submitted to a specific device object.

## Declaration

```swift
func makeCaptureScope(device: any MTLDevice) -> any MTLCaptureScope
```

## Parameters

- **device** — The device object whose commands you want to capture.

## Discussion

The capture scope captures commands in command buffers created on any command queues created by the device object.

## See also

### Creating a capture scope
- [makeCaptureScope(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(commandqueue:)-1rozd) — Creates a capture scope for commands submitted to a specific command queue.
- [defaultCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturemanager/defaultcapturescope) — The capture scope to use when a capture is initiated in Xcode.
