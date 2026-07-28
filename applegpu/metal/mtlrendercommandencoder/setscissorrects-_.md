# setScissorRects(_:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 12.0, macOS 10.13, tvOS 14.5, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrects(_:)>

Configures multiple rectangles for the fragment scissor test.

## Declaration

```swift
func setScissorRects(_ scissorRects: [MTLScissorRect])
```

## Parameters

- **scissorRects** — An array of [MTLScissorRect](https://developer.apple.com/documentation/metal/mtlscissorrect) instances the command applies to the render pipeline for clipping.

## Discussion

The rendering pipeline discards any fragments that lie outside the scissor rectangle. The default scissor rectangle is the same size as the current render attachment, with its origin coordinates in the upper-left corner at `(0, 0)`.

Use this method to configure a different scissor rectangle for multiple viewports you configure with the [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) method. Multiple viewports give your app the ability to draw into separate areas of an image with a single draw call. You can either set a single scissor rectangle for all viewports with the [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)) method, or set each viewport’s rectangle with this method.

> **Important:**
>  The number of scissor rectangles you pass to this method needs to match the number of viewports you configure with the [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) method.

The maximum number of viewports and scissor rectangles a GPU supports varies by device family. For more information, see [MTLGPUFamily](https://developer.apple.com/documentation/metal/mtlgpufamily) and [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions).

The rendering pipeline sends each primitive to a single viewport and its associated scissor rectangle. You can select which viewport each primitive uses in your vertex shader by adding the `[[viewport_array_index]]` attribute to an output value.

> **Note:**
>  You can change the render pass’s scissor rectangle configuration by calling this method again or by calling the [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)) method.

The [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)) method is equivalent to calling this method with a single element in the `scissorRects` array.

## See also

### Configuring viewport and scissor behavior
- [setViewport(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewport(_:)) — Configures the render pipeline with a viewport that applies a transformation and a clipping rectangle.
- [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) — Configures the render pipeline with multiple viewports that apply transformations and clipping rectangles.
- [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)) — Configures a rectangle for the fragment scissor test.
