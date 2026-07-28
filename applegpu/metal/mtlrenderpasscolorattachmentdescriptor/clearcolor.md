# clearColor

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor/clearcolor>

The color to use when clearing the color attachment.

## Declaration

```swift
var clearColor: MTLClearColor { get set }
```

## Discussion

If the [loadAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/loadaction) property of the attachment is set to [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear), then at the start of a render pass, the GPU fills the texture with the value stored in the [clearColor](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor/clearcolor) property. Otherwise, the GPU ignores the [clearColor](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor/clearcolor) property.

The [clearColor](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor/clearcolor) property represents a set of RGBA components. The default value is `(0.0, 0.0, 0.0, 1.0)` (black). Use the [MTLClearColorMake(_:_:_:_:)](https://developer.apple.com/documentation/metal/mtlclearcolormake(_:_:_:_:)) function to construct an [MTLClearColor](https://developer.apple.com/documentation/metal/mtlclearcolor) value.

## See also

### Specifying clearing value
- [MTLClearColorMake(_:_:_:_:)](https://developer.apple.com/documentation/metal/mtlclearcolormake(_:_:_:_:)) — Returns a color value used to clear a color attachment.
