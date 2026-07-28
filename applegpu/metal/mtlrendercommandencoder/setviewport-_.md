# setViewport(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewport(_:)>

Configures the render pipeline with a viewport that applies a transformation and a clipping rectangle.

## Declaration

```swift
func setViewport(_ viewport: MTLViewport)
```

## Parameters

- **viewport** — An [MTLViewport](https://developer.apple.com/documentation/metal/mtlviewport) instance the command applies to the render pipeline for transformations and clipping.

## Discussion

The render pipeline linearly maps vertex positions from normalized device coordinates to viewport coordinates by applying a viewport during the rasterization stage. It applies the transform first and then rasterizes the primitive while clipping any fragments outside the scissor rectangle (see [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:))) or the render target’s extents.

The viewport’s [originX](https://developer.apple.com/documentation/metal/mtlviewport/originx) and [originY](https://developer.apple.com/documentation/metal/mtlviewport/originy) properties, which default to `0.0`, represent the number of pixels from the top-left corner of the render target. Positive [originX](https://developer.apple.com/documentation/metal/mtlviewport/originx) values go to the right and positive [originY](https://developer.apple.com/documentation/metal/mtlviewport/originy) values go downward. The default values for its [width](https://developer.apple.com/documentation/metal/mtlviewport/width) and [height](https://developer.apple.com/documentation/metal/mtlviewport/height) properties are the render target’s width and height, respectively. The default values for its [znear](https://developer.apple.com/documentation/metal/mtlviewport/znear) and [zfar](https://developer.apple.com/documentation/metal/mtlviewport/zfar) properties are `0.0` and `1.0`, respectively, which you can flip.

> **Note:**
>  You can change the render pass’s viewport configuration by calling this method again, or by calling the [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) method.

## See also

### Configuring viewport and scissor behavior
- [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) — Configures the render pipeline with multiple viewports that apply transformations and clipping rectangles.
- [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)) — Configures a rectangle for the fragment scissor test.
- [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrects(_:)) — Configures multiple rectangles for the fragment scissor test.
