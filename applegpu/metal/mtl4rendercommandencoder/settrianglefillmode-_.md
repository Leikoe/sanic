# setTriangleFillMode(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/settrianglefillmode(_:)>

Configures how subsequent draw commands rasterize triangle and triangle strip primitives.

## Declaration

```swift
func setTriangleFillMode(_ fillMode: MTLTriangleFillMode)
```

## Parameters

- **fillMode** — [MTLTriangleFillMode](https://developer.apple.com/documentation/metal/mtltrianglefillmode) the render pass applies to draw commands that rasterize triangles or triangle strips.

## See also

### Configuring rendering behavior
- [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setfrontfacing(_:)) — Configures the vertex winding order that determines which face of a geometric primitive is the front one.
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcullmode(_:)) — Controls whether Metal culls front facing primitives, back facing primitives, or culls no primitives at all.
