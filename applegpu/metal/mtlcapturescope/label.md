# label

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcapturescope/label>

A string that helps you identify the capture scope.

## Declaration

```swift
var label: String? { get set }
```

## Discussion

Setting a capture scope’s label makes it easier to find in Xcode. See [Creating and using custom capture scopes](https://developer.apple.com/documentation/Xcode/Creating-and-using-custom-capture-scopes) for more information.

## See also

### Identifying the capture scope
- [device](https://developer.apple.com/documentation/metal/mtlcapturescope/device) — The device object from which you created the capture scope.
- [commandQueue](https://developer.apple.com/documentation/metal/mtlcapturescope/commandqueue) — The command queue that this capture scope uses to limit which commands are recorded.
