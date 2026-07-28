# setScissorRect(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)>

Configures a rectangle for the fragment scissor test.

## Declaration

```swift
func setScissorRect(_ rect: MTLScissorRect)
```

## Parameters

- **rect** — An [MTLScissorRect](https://developer.apple.com/documentation/metal/mtlscissorrect) instance that represents a rectangle that needs to lie completely within the current render attachment.

## Discussion

The rendering pipeline discards any fragments that lie outside the scissor rectangle.

The default scissor rectangle is the same size as the current render attachment, with its origin coordinates in the upper-left corner at `(0, 0)`.

> **Note:**
>  You can change the render pass’s scissor rectangle configuration by calling this method again or by calling the [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrects(_:)) method.

## See also

### Configuring viewport and scissor behavior
- [setViewport(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewport(_:)) — Configures the render pipeline with a viewport that applies a transformation and a clipping rectangle.
- [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) — Configures the render pipeline with multiple viewports that apply transformations and clipping rectangles.
- [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrects(_:)) — Configures multiple rectangles for the fragment scissor test.
