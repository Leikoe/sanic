# defaultCaptureScope

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturemanager/defaultcapturescope>

The capture scope to use when a capture is initiated in Xcode.

## Declaration

```swift
var defaultCaptureScope: (any MTLCaptureScope)? { get set }
```

## Discussion

Use this property to specify a default capture scope for Xcode to use when the user presses the capture button. You can still long-press the button to select a different capture scope.

The default value is `nil.` When the value is `nil`, the capture scope is defined by drawable presentation boundaries; such as those created by calls to [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)) or [present()](https://developer.apple.com/documentation/metal/mtldrawable/present()).

## See also

### Creating a capture scope
- [makeCaptureScope(device:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(device:)) — Creates a capture scope for commands submitted to a specific device object.
- [makeCaptureScope(commandQueue:)](https://developer.apple.com/documentation/metal/mtlcapturemanager/makecapturescope(commandqueue:)-1rozd) — Creates a capture scope for commands submitted to a specific command queue.
