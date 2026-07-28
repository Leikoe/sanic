# setStencilReferenceValue(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)>

Configures the same comparison value for front- and back-facing primitives.

## Declaration

```swift
func setStencilReferenceValue(_ referenceValue: UInt32)
```

## Parameters

- **referenceValue** — A stencil test comparison value the render pipeline applies to both front- and back-facing primitives.

## Discussion

The command sets the same reference value for front- and back-facing primitives (see [stencilCompareFunction](https://developer.apple.com/documentation/metal/mtlstencildescriptor/stencilcomparefunction), [frontFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/frontfacestencil), and [backFaceStencil](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/backfacestencil)). This reference value applies to the stencil state you set with the [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstencilstate(_:)) method.

The render pass’s default reference value for the front and back stencil compare function is `0`.

## See also

### Configuring depth and stencil behavior
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstencilstate(_:)) — Configures the combined depth and stencil state.
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthbias(_:slopescale:clamp:)) — Configures the adjustments a render pass applies to depth values from fragment functions by a scaling factor and bias.
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthclipmode(_:)) — Configures how the render pipeline handles fragments outside the near and far planes of the view frustum.
- [setDepthTestBounds(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthtestbounds(_:)) — Configures the range for depth bounds testing.
- [setStencilReferenceValues(front:back:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalues(front:back:)) — Configures different comparison values for front- and back-facing primitives.
