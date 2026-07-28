# setTriangleFillMode(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settrianglefillmode(_:)>

Configures how subsequent draw commands rasterize triangle and triangle strip primitives.

## Declaration

```swift
func setTriangleFillMode(_ fillMode: MTLTriangleFillMode)
```

## Parameters

- **fillMode** — A triangle filling mode the render pass applies to draw commands that rasterize triangles or triangle strips.

## Discussion

The render pass’s default mode is [MTLTriangleFillMode.fill](https://developer.apple.com/documentation/metal/mtltrianglefillmode/fill).

## See also

### Configuring rendering behavior
- [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfrontfacing(_:)) — Configures which face of a primitive, such as a triangle, is the front.
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcullmode(_:)) — Configures how the render pipeline determines which primitives to remove.
