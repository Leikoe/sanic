# setColorStoreAction(_:index:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreaction(_:index:)>

Configures the store action for a color attachment.

## Declaration

```swift
func setColorStoreAction(_ storeAction: MTLStoreAction, index colorAttachmentIndex: Int)
```

## Parameters

- **storeAction** — A store action for the color attachment that can’t be [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown).
- **colorAttachmentIndex** — The index of a color attachment.

## Discussion

This method changes the render command encoder’s store action for a color attachment. You can assign the default store action for a color attachment by configuring the [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) property of its [MTLRenderPassColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpasscolorattachmentdescriptor) (see [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) and its [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/colorattachments) property).

> **Important:**
>  You need to call this method before calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method, but only for color attachments with a [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) property equal to [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown).

## See also

### Configuring the actions for attachments
- [setColorStoreActionOptions(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreactionoptions(_:index:)) — Configures the store action options for a color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreaction(_:)) — Configures the store action for the depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreactionoptions(_:)) — Configures the store action options for the depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreaction(_:)) — Configures the store action for the stencil attachment.
- [setStencilStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreactionoptions(_:)) — Configures the store action options for the stencil attachment.
