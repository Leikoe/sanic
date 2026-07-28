# resolveLevel

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvelevel>

The mipmap level of the texture used for the multisample resolve action.

## Declaration

```swift
var resolveLevel: Int { get set }
```

## Discussion

If the value of [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) is set to [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) or [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve), set this property to point to a mipmap in the resolve texture.

The default value is `0`.

## See also

### Specifying the texture to resolve multisample data
- [resolveTexture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvetexture) — The destination texture used when resolving multisampled texture data into single sample values.
- [resolveSlice](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolveslice) — The slice of the texture used for the multisample resolve action.
- [resolveDepthPlane](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvedepthplane) — The depth plane of the texture used for the multisample resolve action.
