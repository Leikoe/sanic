# setFrontFacing(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setfrontfacing(_:)>

Configures the vertex winding order that determines which face of a geometric primitive is the front one.

## Declaration

```swift
func setFrontFacing(_ frontFacingWinding: MTLWinding)
```

## Parameters

- **frontFacingWinding** — A [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) value that determines which side of a primitive the render pipeline interprets as front facing.

## See also

### Configuring rendering behavior
- [setTriangleFillMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/settrianglefillmode(_:)) — Configures how subsequent draw commands rasterize triangle and triangle strip primitives.
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcullmode(_:)) — Controls whether Metal culls front facing primitives, back facing primitives, or culls no primitives at all.
