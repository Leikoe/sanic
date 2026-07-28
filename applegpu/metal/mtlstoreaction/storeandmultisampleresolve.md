# MTLStoreAction.storeAndMultisampleResolve

*Case · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve>

The GPU stores the multisample data to the multisample texture, resolves the data to a sample per pixel, and stores the data to the resolve texture.

## Declaration

```swift
case storeAndMultisampleResolve
```

## See also

### Store actions
- [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare) — The GPU has permission to discard the rendered contents of the attachment at the end of the render pass, replacing them with arbitrary data.
- [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) — The GPU stores the rendered contents to the texture.
- [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) — The GPU resolves the multisampled data to one sample per pixel and stores the data to the resolve texture, discarding the multisample data afterwards.
- [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) — The system selects a store action when it encodes the render pass.
- [MTLStoreAction.customSampleDepthStore](https://developer.apple.com/documentation/metal/mtlstoreaction/customsampledepthstore) — The GPU stores depth data in a sample-position–agnostic representation.
