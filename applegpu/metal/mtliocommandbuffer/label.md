# label

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer/label>

An optional name for the input/output command buffer.

## Declaration

```swift
var label: String? { get set }
```

## See also

### Debugging a command buffer
- [pushDebugGroup(_:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/pushdebuggroup(_:)) — Sets the current name for this input/output command encoder by adding it to the top of the debug name stack.
- [popDebugGroup()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/popdebuggroup()) — Restores the previous name for this input/output command encoder by removing the top item of the debug name stack.
