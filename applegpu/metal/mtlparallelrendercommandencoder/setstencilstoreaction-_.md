# setStencilStoreAction(_:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setstencilstoreaction(_:)>

Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given stencil attachment.

## Declaration

```swift
func setStencilStoreAction(_ storeAction: MTLStoreAction)
```

## Parameters

- **storeAction** — The desired store action for the stencil attachment. This value can’t be [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown).

## Discussion

If the store action for the given stencil attachment was set to [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) when the parallel render command encoder was created, you need to call this method to specify another store action before you call the [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method.

## See also

### Setting render pass state
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setcolorstoreaction(_:index:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given color attachment.
- [setColorStoreActionOptions(_:index:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setcolorstoreactionoptions(_:index:)) — Specifies known store action options for a given color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setdepthstoreaction(_:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setdepthstoreactionoptions(_:)) — Specifies known store action options for a given depth attachment.
- [setStencilStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setstencilstoreactionoptions(_:)) — Specifies known store action options for a given stencil attachment.
