# resolveTexture

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvetexture>

The destination texture used when resolving multisampled texture data into single sample values.

## Declaration

```swift
var resolveTexture: (any MTLTexture)? { get set }
```

## Discussion

If the [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) value is set to [MTLStoreAction.multisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/multisampleresolve) or [MTLStoreAction.storeAndMultisampleResolve](https://developer.apple.com/documentation/metal/mtlstoreaction/storeandmultisampleresolve), then the [resolveTexture](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvetexture) value needs to point to a valid texture. Otherwise, Metal ignores this property.

## See also

### Specifying the texture to resolve multisample data
- [resolveLevel](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvelevel) — The mipmap level of the texture used for the multisample resolve action.
- [resolveSlice](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolveslice) — The slice of the texture used for the multisample resolve action.
- [resolveDepthPlane](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/resolvedepthplane) — The depth plane of the texture used for the multisample resolve action.
