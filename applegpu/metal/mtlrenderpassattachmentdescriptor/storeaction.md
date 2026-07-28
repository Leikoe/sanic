# storeAction

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction>

The action performed by this attachment at the end of a rendering pass for a render command encoder.

## Declaration

```swift
var storeAction: MTLStoreAction { get set }
```

## Discussion

If your app doesn’t need the data in the texture after completing the rendering pass, use the [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare) action. Otherwise, use the [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store) action if the texture is directly stored or the [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) action if the texture is a multisampled texture. In some feature sets, you can use the [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve) action to store and resolve the texture in a single rendering pass. For more information, see:

- [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf)

- [Metal feature set tables (Numbers)](https://developer.apple.com/metal/metal-feature-set-tables.zip)

When the store action is either [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) or [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve), the [resolveTexture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvetexture) property needs to be set to the texture to use as the target for the resolve action. Use the [resolveLevel](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvelevel), [resolveSlice](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolveslice), and [resolveDepthPlane](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvedepthplane) properties to specify the mipmap level, cube slice, and depth plane of the resolve texture, respectively.

For color render targets, the default value is [MTLStoreAction.store](https://developer.apple.com/documentation/metal/mtlstoreaction/store). For depth or stencil render targets, the default value is [MTLStoreAction.dontCare](https://developer.apple.com/documentation/metal/mtlstoreaction/dontcare).

## See also

### Specifying rendering pass actions
- [loadAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/loadaction) — The action performed by this attachment at the start of a rendering pass for a render command encoder.
- [storeActionOptions](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeactionoptions) — The options that modify the store action performed by this attachment.
