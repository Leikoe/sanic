# MTLStoreAction.unknown

*Case · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoreaction/unknown>

The system selects a store action when it encodes the render pass.

## Declaration

```swift
case unknown
```

## Discussion

Only apply this action if you can’t determine the store action when you create the render pass descriptor. You need to specify a store action before you finish encoding commands into the render command encoder. Refer to the [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) and [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) protocol references for further information.

## See also

### Store actions
- [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare) — The GPU has permission to discard the rendered contents of the attachment at the end of the render pass, replacing them with arbitrary data.
- [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) — The GPU stores the rendered contents to the texture.
- [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) — The GPU resolves the multisampled data to one sample per pixel and stores the data to the resolve texture, discarding the multisample data afterwards.
- [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) — The GPU stores the multisample data to the multisample texture, resolves the data to a sample per pixel, and stores the data to the resolve texture.
- [MTLStoreAction.customSampleDepthStore](https://developer.apple.com/documentation/metal/mtlstoreaction/customsampledepthstore) — The GPU stores depth data in a sample-position–agnostic representation.
