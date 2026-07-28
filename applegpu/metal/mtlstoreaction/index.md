# MTLStoreAction

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoreaction>

Types of actions performed for an attachment at the end of a rendering pass.

## Declaration

```swift
enum MTLStoreAction
```

## Topics

### Store actions
- [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare) — The GPU has permission to discard the rendered contents of the attachment at the end of the render pass, replacing them with arbitrary data.
- [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) — The GPU stores the rendered contents to the texture.
- [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) — The GPU resolves the multisampled data to one sample per pixel and stores the data to the resolve texture, discarding the multisample data afterwards.
- [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) — The GPU stores the multisample data to the multisample texture, resolves the data to a sample per pixel, and stores the data to the resolve texture.
- [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) — The system selects a store action when it encodes the render pass.
- [MTLStoreAction.customSampleDepthStore](https://developer.apple.com/documentation/metal/mtlstoreaction/customsampledepthstore) — The GPU stores depth data in a sample-position–agnostic representation.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlstoreaction/init(rawvalue:))

## See also

### Encoding a render pass in parallel
- [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) — An instance that splits up a single render pass so that it can be simultaneously encoded from multiple threads.
- [MTLLoadAction](https://developer.apple.com/documentation/metal/mtlloadaction) — Types of actions performed for an attachment at the start of a rendering pass.
- [MTLStoreActionOptions](https://developer.apple.com/documentation/metal/mtlstoreactionoptions) — Options that modify a store action.
