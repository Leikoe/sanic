# setColorStoreActionOptions(_:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreactionoptions(_:index:)>

Configures the store action options for a color attachment.

## Declaration

```swift
func setColorStoreActionOptions(_ storeActionOptions: MTLStoreActionOptions, index colorAttachmentIndex: Int)
```

## Parameters

- **storeActionOptions** — Additional options for the store action of a color attachment.
- **colorAttachmentIndex** — The index of a color attachment.

## See also

### Configuring the actions for attachments
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreaction(_:index:)) — Configures the store action for a color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreaction(_:)) — Configures the store action for the depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreactionoptions(_:)) — Configures the store action options for the depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreaction(_:)) — Configures the store action for the stencil attachment.
- [setStencilStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreactionoptions(_:)) — Configures the store action options for the stencil attachment.
