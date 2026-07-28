# MTLCaptureDestination

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturedestination>

The kinds of destinations for captured command data.

## Declaration

```swift
enum MTLCaptureDestination
```

## Topics

### Choosing a destination
- [MTLCaptureDestination.developerTools](https://developer.apple.com/documentation/metal/mtlcapturedestination/developertools) — An option specifying that data should be captured to Xcode and that execution should stop in Xcode after the data is captured.
- [MTLCaptureDestination.gpuTraceDocument](https://developer.apple.com/documentation/metal/mtlcapturedestination/gputracedocument) — An option specifying that the captured command data should be saved to a GPU trace document.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcapturedestination/init(rawvalue:))

## See also

### Frame capture
- [MTLCaptureDescriptor](https://developer.apple.com/documentation/metal/mtlcapturedescriptor) — A configuration for a Metal capture session.
- [MTLCaptureManager](https://developer.apple.com/documentation/metal/mtlcapturemanager) — An instance you use to capture Metal command data in your app.
- [MTLCaptureScope](https://developer.apple.com/documentation/metal/mtlcapturescope) — A type that can programmatically customize a GPU frame capture.
