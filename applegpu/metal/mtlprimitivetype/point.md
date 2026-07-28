# MTLPrimitiveType.point

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlprimitivetype/point>

Rasterize a point at each vertex. The vertex shader needs to provide `[[point_size]]`, or the point size is undefined.

## Declaration

```swift
case point
```

## See also

### Geometric primitive types
- [MTLPrimitiveType.line](https://developer.apple.com/documentation/metal/mtlprimitivetype/line) — Rasterize a line between each separate pair of vertices, resulting in a series of unconnected lines. If there are an odd number of vertices, the last vertex is ignored.
- [MTLPrimitiveType.lineStrip](https://developer.apple.com/documentation/metal/mtlprimitivetype/linestrip) — Rasterize a line between each pair of adjacent vertices, resulting in a series of connected lines (also called a polyline).
- [MTLPrimitiveType.triangle](https://developer.apple.com/documentation/metal/mtlprimitivetype/triangle) — For every separate set of three vertices, rasterize a triangle. If the number of vertices is not a multiple of three, either one or two vertices is ignored.
- [MTLPrimitiveType.triangleStrip](https://developer.apple.com/documentation/metal/mtlprimitivetype/trianglestrip) — For every three adjacent vertices, rasterize a triangle.
