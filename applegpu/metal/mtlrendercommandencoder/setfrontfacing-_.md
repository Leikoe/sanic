# setFrontFacing(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfrontfacing(_:)>

Configures which face of a primitive, such as a triangle, is the front.

## Declaration

```swift
func setFrontFacing(_ frontFacingWinding: MTLWinding)
```

## Parameters

- **frontFacingWinding** — An [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) value that configures how the render pipeline defines which side of a primitive is its front.

## Discussion

The render pass’s default front-facing mode is [MTLWinding.clockwise](https://developer.apple.com/documentation/metal/mtlwinding/clockwise).

The winding direction of a primitive determines whether the render pass culls it (see [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcullmode(_:))).

## See also

### Configuring rendering behavior
- [setTriangleFillMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settrianglefillmode(_:)) — Configures how subsequent draw commands rasterize triangle and triangle strip primitives.
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcullmode(_:)) — Configures how the render pipeline determines which primitives to remove.
