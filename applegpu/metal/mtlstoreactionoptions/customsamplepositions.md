# customSamplePositions

*Type Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoreactionoptions/customsamplepositions>

An option that stores data in a sample-position–agnostic representation.

## Declaration

```swift
static var customSamplePositions: MTLStoreActionOptions { get }
```

## Discussion

Set this option only on an [MTLRenderPassColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor) or [MTLRenderPassDepthAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdepthattachmentdescriptor) instance. Setting this option on an [MTLRenderPassStencilAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassstencilattachmentdescriptor) instance or combining it with a nonstore [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) value results in a runtime error.

Set this action when you need to read the data in a subsequent render pass or blit operation that is unaware of the programmable sample positions used to generate the data. You should set this option when, for example, reading per-sample data within a fragment function that uses different programmable sample positions.

If you specify this action, Metal may decompress the depth render target and store the resulting data in its decompressed form. If you don’t change programmable sample positions in a subsequent render pass, use [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) instead to improve performance.
