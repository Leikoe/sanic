# MTLIndexType

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindextype>

The index type for an index buffer that references vertices of geometric primitives.

## Declaration

```swift
enum MTLIndexType
```

## Topics

### Index types
- [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) — A 16-bit unsigned integer used as a primitive index.
- [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32) — A 32-bit unsigned integer used as a primitive index.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlindextype/init(rawvalue:))

## See also

### Encoding a render pass
- [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTLTriangleFillMode](https://developer.apple.com/documentation/metal/mtltrianglefillmode) — Specifies how to rasterize triangle and triangle strip primitives.
- [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) — The vertex winding rule that determines a front-facing primitive.
- [MTLCullMode](https://developer.apple.com/documentation/metal/mtlcullmode) — The mode that determines whether to perform culling and which type of primitive to cull.
- [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) — The geometric primitive type for drawing commands.
- [MTLDepthClipMode](https://developer.apple.com/documentation/metal/mtldepthclipmode) — The mode that determines how to deal with fragments outside of the near or far planes.
- [MTLVisibilityResultMode](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode) — The mode that determines what, if anything, the GPU writes to the results buffer, after the GPU executes the render pass.
- [MTLVisibilityResultType](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype) — This enumeration controls if Metal accumulates visibility results between render encoders or resets them.
