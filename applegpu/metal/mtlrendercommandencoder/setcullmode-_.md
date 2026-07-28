# setCullMode(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcullmode(_:)>

Configures how the render pipeline determines which primitives to remove.

## Declaration

```swift
func setCullMode(_ cullMode: MTLCullMode)
```

## Parameters

- **cullMode** — An [MTLCullMode](https://developer.apple.com/documentation/metal/mtlcullmode) value that configures how the render pipeline determines which primitives to remove from the pipeline.

## Discussion

This method configures which primitives the render pipeline removes, if any, based on the direction of each primitive’s face relative to the scene’s camera. For example, you can correctly cull hidden surfaces on some geometric models, such as a sphere made of filled triangles, if it uses orientable surfaces. A surface is *orientable* if its primitives consistently use the same ordering for its vertices. Metal defines vertex ordering with the [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) type, which includes [MTLWinding.clockwise](https://developer.apple.com/documentation/metal/mtlwinding/clockwise) and [MTLWinding.counterClockwise](https://developer.apple.com/documentation/metal/mtlwinding/counterclockwise). You can tell the render pipeline which direction your primitives face by calling the [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfrontfacing(_:)) method, which affects the primitives the culling mode removes.

The render pass’s default culling mode is [MTLCullMode.none](https://developer.apple.com/documentation/metal/mtlcullmode/none).

## See also

### Configuring rendering behavior
- [setTriangleFillMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settrianglefillmode(_:)) — Configures how subsequent draw commands rasterize triangle and triangle strip primitives.
- [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfrontfacing(_:)) — Configures which face of a primitive, such as a triangle, is the front.
