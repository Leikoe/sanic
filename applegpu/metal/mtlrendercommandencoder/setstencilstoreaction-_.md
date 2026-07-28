# setStencilStoreAction(_:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreaction(_:)>

Configures the store action for the stencil attachment.

## Declaration

```swift
func setStencilStoreAction(_ storeAction: MTLStoreAction)
```

## Parameters

- **storeAction** — A store action for the stencil attachment that can’t be [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown).

## Discussion

This method changes the render command encoder’s store action for the stencil attachment. You can assign the default store action for the stencil attachment by configuring the [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) property of its [MTLRenderPassStencilAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassstencilattachmentdescriptor) (see [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) and its [stencilAttachment](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/stencilattachment) property).

> **Important:**
>  You need to call this method before calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method, but only if the stencil attachment’s [storeAction](https://developer.apple.com/documentation/metal/mtlrenderpassattachmentdescriptor/storeaction) property is equal to [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown).

## See also

### Configuring the actions for attachments
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreaction(_:index:)) — Configures the store action for a color attachment.
- [setColorStoreActionOptions(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreactionoptions(_:index:)) — Configures the store action options for a color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreaction(_:)) — Configures the store action for the depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreactionoptions(_:)) — Configures the store action options for the depth attachment.
- [setStencilStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreactionoptions(_:)) — Configures the store action options for the stencil attachment.
