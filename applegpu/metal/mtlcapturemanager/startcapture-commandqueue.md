# startCapture(commandQueue:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(commandqueue:)>

Starts capturing any of your app’s Metal commands that are executed by the command queue.

## Declaration

```swift
func startCapture(commandQueue: any MTLCommandQueue)
```

## Parameters

- **commandQueue** — The command queue whose commands you want to capture.

## See also

### Starting capture
- [startCapture(with:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(with:)) — Starts capturing any of your app’s Metal commands, with the capture session defined by a descriptor object.
- [startCapture(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(device:)) — Starts capturing any of your app’s Metal commands that are executed by the device object.
- [startCapture(scope:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(scope:)) — Starts capturing any of your app’s Metal commands that are in the specified capture scope.
