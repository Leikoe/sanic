# MTLCaptureManager

*Class · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager>

An instance you use to capture Metal command data in your app.

## Declaration

```swift
class MTLCaptureManager
```

## Overview

A capture manager works with the frame capture feature to:

- Capture data about Metal commands programmatically. See [Capturing a Metal workload programmatically](https://developer.apple.com/documentation/Xcode/Capturing-a-Metal-workload-programmatically).

- Only capture commands that apply to a specific [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice), command queue, or [MTLCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturescope) instance.

- Assign a default [MTLCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturescope) instance for captures you create in Xcode by clicking the Capture GPU workload button in the debug bar, which has an icon with the Metal logo.

The Metal debugger requires you to enable GPU Frame Capture in your project settings; see [Capturing a Metal workload in Xcode](https://developer.apple.com/documentation/Xcode/Capturing-a-Metal-workload-in-Xcode).

> **Important:**
>  The capture manager records commands within the [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance that you create and commit while the capture session is active.

For more information about Metal frame capture, see [Metal debugger](https://developer.apple.com/documentation/Xcode/Metal-debugger).

## Topics

### Obtaining the shared capture manager
- [shared()](https://developer.apple.com/documentation/metal/mtlcapturemanager/shared()) — Provides the shared capture manager for your Metal app.

### Querying support for a capture destination
- [supportsDestination(_:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/supportsdestination(_:)) — Checks to see whether a particular capture destination is supported.

### Creating a capture scope
- [makeCaptureScope(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(device:)) — Creates a capture scope for commands submitted to a specific device object.
- [makeCaptureScope(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(commandqueue:)-1rozd) — Creates a capture scope for commands submitted to a specific command queue.
- [defaultCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturemanager/defaultcapturescope) — The capture scope to use when a capture is initiated in Xcode.

### Starting capture
- [startCapture(with:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(with:)) — Starts capturing any of your app’s Metal commands, with the capture session defined by a descriptor object.
- [startCapture(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(device:)) — Starts capturing any of your app’s Metal commands that are executed by the device object.
- [startCapture(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(commandqueue:)) — Starts capturing any of your app’s Metal commands that are executed by the command queue.
- [startCapture(scope:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/startcapture(scope:)) — Starts capturing any of your app’s Metal commands that are in the specified capture scope.

### Stopping capture
- [stopCapture()](https://developer.apple.com/documentation/metal/mtlcapturemanager/stopcapture()) — Stops capturing Metal commands.

### Monitoring capture
- [isCapturing](https://developer.apple.com/documentation/metal/mtlcapturemanager/iscapturing) — A Boolean value that indicates whether Metal commands are being captured.

### Instance Methods
- [makeCaptureScope(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(commandqueue:)-9wie3)

## See also

### Frame capture
- [MTLCaptureDescriptor](https://developer.apple.com/documentation/metal/mtlcapturedescriptor) — A configuration for a Metal capture session.
- [MTLCaptureDestination](https://developer.apple.com/documentation/metal/mtlcapturedestination) — The kinds of destinations for captured command data.
- [MTLCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturescope) — A type that can programmatically customize a GPU frame capture.
