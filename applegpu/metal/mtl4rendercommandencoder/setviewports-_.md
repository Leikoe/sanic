# setViewports(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewports(_:)>

Sets an array of viewports to transform vertices from normalized device coordinates to window coordinates.

## Declaration

```swift
func setViewports(_ viewports: [MTLViewport])
```

## Parameters

- **viewports** — A Swift array of [MTLViewport](https://developer.apple.com/documentation/metal/mtlviewport) elements.

## Discussion

Metal clips fragments that lie outside of the viewport, and optionally clamps fragments outside of z-near/z-far range, depending on the value you assign to [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthclipmode(_:)).

Metal selects the viewport to use from the `[[ viewport_array_index ]]` attribute you specify in the pipeline state’s vertex shader function in the Metal Shading Language.

## See also

### Configuring viewport and scissor behavior
- [setViewport(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewport(_:)) — Sets the viewport which that transforms vertices from normalized device coordinates to window coordinates.
- [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrect(_:)) — Sets a scissor rectangle to discard fragments outside a specific area.
- [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrects(_:)) — Sets an array of scissor rectangles for a fragment scissor test.
