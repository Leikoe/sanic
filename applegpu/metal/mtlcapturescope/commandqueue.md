# commandQueue

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturescope/commandqueue>

The command queue that this capture scope uses to limit which commands are recorded.

## Declaration

```swift
var commandQueue: (any MTLCommandQueue)? { get }
```

## Discussion

This value is only available if you created the capture scope by calling the [makeCaptureScope(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(commandqueue:)-1rozd) method. Otherwise, the value is `nil`.

## See also

### Identifying the capture scope
- [label](https://developer.apple.com/documentation/metal/mtlcapturescope/label) — A string that helps you identify the capture scope.
- [device](https://developer.apple.com/documentation/metal/mtlcapturescope/device) — The device object from which you created the capture scope.
