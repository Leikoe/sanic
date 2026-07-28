# setColorStoreAction(_:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcolorstoreaction(_:index:)>

Configures the store action for a color attachment.

## Declaration

```swift
func setColorStoreAction(_ storeAction: MTLStoreAction, index colorAttachmentIndex: Int)
```

## Parameters

- **storeAction** — A store action for the color attachment that can’t be [MTLStoreAction.unknown](https://developer.apple.com/documentation/metal/mtlstoreaction/unknown).
- **colorAttachmentIndex** — The index of a color attachment.

## See also

### Configuring the actions for attachments
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthstoreaction(_:)) — Configures the store action for the depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilstoreaction(_:)) — Configures the store action for the stencil attachment.
