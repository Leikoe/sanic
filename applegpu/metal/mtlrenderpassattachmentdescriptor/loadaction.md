# loadAction

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/loadaction>

The action performed by this attachment at the start of a rendering pass for a render command encoder.

## Declaration

```swift
var loadAction: MTLLoadAction { get set }
```

## Discussion

If your app renders all pixels of the render target for a given frame, use the [MTLLoadAction.dontCare](https://developer.apple.com/documentation/metal/mtlloadaction/dontcare) action, which allows the GPU to avoid loading the existing contents of the texture. Otherwise, use the [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear) action to clear the previous contents of the render target or the [MTLLoadAction.load](https://developer.apple.com/documentation/metal/mtlloadaction/load) action to preserve them. The [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear) action also avoids the cost of loading the existing texture contents, but it still incurs the cost of filling the destination with a clear color.

For color render targets, the default value is [MTLLoadAction.dontCare](https://developer.apple.com/documentation/metal/mtlloadaction/dontcare). For depth or stencil render targets, the default value is [MTLLoadAction.clear](https://developer.apple.com/documentation/metal/mtlloadaction/clear).

## See also

### Specifying rendering pass actions
- [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) — The action performed by this attachment at the end of a rendering pass for a render command encoder.
- [storeActionOptions](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeactionoptions) — The options that modify the store action performed by this attachment.
