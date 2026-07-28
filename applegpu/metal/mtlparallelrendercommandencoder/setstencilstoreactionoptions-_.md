# setStencilStoreActionOptions(_:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setstencilstoreactionoptions(_:)>

Specifies known store action options for a given stencil attachment.

## Declaration

```swift
func setStencilStoreActionOptions(_ storeActionOptions: MTLStoreActionOptions)
```

## Parameters

- **storeActionOptions** — The additional store action options for the stencil attachment.

## See also

### Setting render pass state
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setcolorstoreaction(_:index:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given color attachment.
- [setColorStoreActionOptions(_:index:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setcolorstoreactionoptions(_:index:)) — Specifies known store action options for a given color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setdepthstoreaction(_:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setdepthstoreactionoptions(_:)) — Specifies known store action options for a given depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/setstencilstoreaction(_:)) — Specifies a known store action to replace the initial [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown) value specified for a given stencil attachment.
