# clearDepth

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor/cleardepth>

The depth to use when clearing the depth attachment.

## Declaration

```swift
var clearDepth: Double { get set }
```

## Discussion

If the [loadAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/loadaction) property of the attachment is set to [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear), then at the start of a render pass, the GPU fills the contents of the attachment with the value stored in the [clearDepth](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor/cleardepth) property. Otherwise, the GPU ignores [clearDepth](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor/cleardepth).

The default value is `1.0`.
