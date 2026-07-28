# startCapture(with:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(with:)>

Starts capturing any of your app’s Metal commands, with the capture session defined by a descriptor object.

## Declaration

```swift
func startCapture(with descriptor: MTLCaptureDescriptor) throws
```

## Parameters

- **descriptor** — A description of the capture session to create.

## See also

### Starting capture
- [startCapture(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(device:)) — Starts capturing any of your app’s Metal commands that are executed by the device object.
- [startCapture(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(commandqueue:)) — Starts capturing any of your app’s Metal commands that are executed by the command queue.
- [startCapture(scope:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(scope:)) — Starts capturing any of your app’s Metal commands that are in the specified capture scope.
