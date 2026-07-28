# MTLPrimitiveType

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlprimitivetype>

The geometric primitive type for drawing commands.

## Declaration

```swift
enum MTLPrimitiveType
```

## Topics

### Geometric primitive types
- [MTLPrimitiveType.point](https://developer.apple.com/documentation/metal/mtlprimitivetype/point) — Rasterize a point at each vertex. The vertex shader needs to provide `[[point_size]]`, or the point size is undefined.
- [MTLPrimitiveType.line](https://developer.apple.com/documentation/metal/mtlprimitivetype/line) — Rasterize a line between each separate pair of vertices, resulting in a series of unconnected lines. If there are an odd number of vertices, the last vertex is ignored.
- [MTLPrimitiveType.lineStrip](https://developer.apple.com/documentation/metal/mtlprimitivetype/linestrip) — Rasterize a line between each pair of adjacent vertices, resulting in a series of connected lines (also called a polyline).
- [MTLPrimitiveType.triangle](https://developer.apple.com/documentation/metal/mtlprimitivetype/triangle) — For every separate set of three vertices, rasterize a triangle. If the number of vertices is not a multiple of three, either one or two vertices is ignored.
- [MTLPrimitiveType.triangleStrip](https://developer.apple.com/documentation/metal/mtlprimitivetype/trianglestrip) — For every three adjacent vertices, rasterize a triangle.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlprimitivetype/init(rawvalue:))

## See also

### Encoding a render pass
- [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTLTriangleFillMode](https://developer.apple.com/documentation/metal/mtltrianglefillmode) — Specifies how to rasterize triangle and triangle strip primitives.
- [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) — The vertex winding rule that determines a front-facing primitive.
- [MTLCullMode](https://developer.apple.com/documentation/metal/mtlcullmode) — The mode that determines whether to perform culling and which type of primitive to cull.
- [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) — The index type for an index buffer that references vertices of geometric primitives.
- [MTLDepthClipMode](https://developer.apple.com/documentation/metal/mtldepthclipmode) — The mode that determines how to deal with fragments outside of the near or far planes.
- [MTLVisibilityResultMode](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode) — The mode that determines what, if anything, the GPU writes to the results buffer, after the GPU executes the render pass.
- [MTLVisibilityResultType](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype) — This enumeration controls if Metal accumulates visibility results between render encoders or resets them.
