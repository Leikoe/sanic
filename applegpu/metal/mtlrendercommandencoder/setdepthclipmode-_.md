# setDepthClipMode(_:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.11, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthclipmode(_:)>

Configures how the render pipeline handles fragments outside the near and far planes of the view frustum.

## Declaration

```swift
func setDepthClipMode(_ depthClipMode: MTLDepthClipMode)
```

## Parameters

- **depthClipMode** — The mode that determines how to handle fragments outside the near and far planes.

## Discussion

You can use depth clipping to ignore fragments outside the z-axis boundaries of a viewing volume.

The render pass’s default clip mode is [MTLDepthClipMode.clip](https://developer.apple.com/documentation/metal/mtldepthclipmode/clip).

## See also

### Configuring depth and stencil behavior
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstencilstate(_:)) — Configures the combined depth and stencil state.
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthbias(_:slopescale:clamp:)) — Configures the adjustments a render pass applies to depth values from fragment functions by a scaling factor and bias.
- [setDepthTestBounds(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthtestbounds(_:)) — Configures the range for depth bounds testing.
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)) — Configures the same comparison value for front- and back-facing primitives.
- [setStencilReferenceValues(front:back:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalues(front:back:)) — Configures different comparison values for front- and back-facing primitives.
