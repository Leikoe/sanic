# setDepthTestBounds(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthtestbounds(_:)>

Configures the range for depth bounds testing.

## Declaration

```swift
func setDepthTestBounds(_ bounds: ClosedRange<Float>)
```

## Parameters

- **bounds** — A closed range the renderer applies to depth bounds testing. The renderer discards fragments with a stored depth that is outside `bounds`.

## Discussion

The render command encoder disables depth bounds testing by default. The render command encoder also disables depth bounds testing when the `bounds` property equals `0.0...1.0`. `bounds.lowerBound` needs to be greater than or equal to `0.0`. `bounds.upperBound` needs to be less than or equal to `1.0`.

## See also

### Configuring depth and stencil behavior
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthstencilstate(_:)) — Configures this encoder with a depth stencil state that applies to your subsequent draw commands.
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthbias(_:slopescale:clamp:)) — Configures the adjustments a render pass applies to depth values from fragment shader functions by a scaling factor and bias.
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthclipmode(_:)) — Controls the behavior for fragments outside of the near or far planes.
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilreferencevalue(_:)) — Configures this encoder with a reference value for stencil testing.
- [setStencilReferenceValue(front:back:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilreferencevalue(front:back:)) — Configures the encoder with different stencil test reference values for front-facing and back-facing primitives.
