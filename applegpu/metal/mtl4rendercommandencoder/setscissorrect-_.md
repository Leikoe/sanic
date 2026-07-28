# setScissorRect(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrect(_:)>

Sets a scissor rectangle to discard fragments outside a specific area.

## Declaration

```swift
func setScissorRect(_ rect: MTLScissorRect)
```

## Parameters

- **rect** — [MTLScissorRect](https://developer.apple.com/documentation/metal/mtlscissorrect) rectangle to specify. This rectangle needs to lie completely within the current render attachment.

## Discussion

Metal performs a scissor test and discards all fragments outside of the scissor rect.

## See also

### Configuring viewport and scissor behavior
- [setViewport(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewport(_:)) — Sets the viewport which that transforms vertices from normalized device coordinates to window coordinates.
- [setViewports(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewports(_:)) — Sets an array of viewports to transform vertices from normalized device coordinates to window coordinates.
- [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrects(_:)) — Sets an array of scissor rectangles for a fragment scissor test.
