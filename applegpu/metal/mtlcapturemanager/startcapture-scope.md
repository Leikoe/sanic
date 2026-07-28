# startCapture(scope:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(scope:)>

Starts capturing any of your app’s Metal commands that are in the specified capture scope.

## Declaration

```swift
func startCapture(scope captureScope: any MTLCaptureScope)
```

## Parameters

- **captureScope** — The capture scope to use.

## See also

### Starting capture
- [startCapture(with:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(with:)) — Starts capturing any of your app’s Metal commands, with the capture session defined by a descriptor object.
- [startCapture(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(device:)) — Starts capturing any of your app’s Metal commands that are executed by the device object.
- [startCapture(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(commandqueue:)) — Starts capturing any of your app’s Metal commands that are executed by the command queue.
