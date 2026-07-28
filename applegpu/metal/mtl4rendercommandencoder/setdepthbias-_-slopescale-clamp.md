# setDepthBias(_:slopeScale:clamp:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthbias(_:slopescale:clamp:)>

Configures the adjustments a render pass applies to depth values from fragment shader functions by a scaling factor and bias.

## Declaration

```swift
func setDepthBias(_ depthBias: Float, slopeScale: Float, clamp: Float)
```

## Parameters

- **depthBias** — A constant bias the render pipeline applies to all fragments.
- **slopeScale** — A bias coefficient that scales with the depth of the primitive relative to the camera.
- **clamp** — A value that limits the bias value the render pipeline can apply to a fragment. Pass a positive or negative value to limit the largest magnitude of a positive or negative bias, respectively. Set this value to `0` to disable bias clamping.

## See also

### Configuring depth and stencil behavior
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthstencilstate(_:)) — Configures this encoder with a depth stencil state that applies to your subsequent draw commands.
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthclipmode(_:)) — Controls the behavior for fragments outside of the near or far planes.
- [setDepthTestBounds(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthtestbounds(_:)) — Configures the range for depth bounds testing.
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilreferencevalue(_:)) — Configures this encoder with a reference value for stencil testing.
- [setStencilReferenceValue(front:back:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilreferencevalue(front:back:)) — Configures the encoder with different stencil test reference values for front-facing and back-facing primitives.
