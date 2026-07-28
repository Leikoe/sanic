# MTLVisibilityResultType

*Enumeration · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlvisibilityresulttype>

This enumeration controls if Metal accumulates visibility results between render encoders or resets them.

## Declaration

```swift
enum MTLVisibilityResultType
```

## Overview

You can specify this property for `MTLRenderCommandEncoders` and for `MTL4RenderCommandEncoders` through their descriptors’ `MTLRenderCommandEncoder/visibilityResultType` and `MTL4RenderCommandEncoder/visibilityResultType` methods.

## Topics

### Enumeration Cases
- [MTLVisibilityResultType.accumulate](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype/accumulate) — Accumulate visibility results data across multiple render passes.
- [MTLVisibilityResultType.reset](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype/reset) — Reset visibility result data when you create a render command encoder.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype/init(rawvalue:))

## See also

### Encoding a render pass
- [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTLTriangleFillMode](https://developer.apple.com/documentation/metal/mtltrianglefillmode) — Specifies how to rasterize triangle and triangle strip primitives.
- [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) — The vertex winding rule that determines a front-facing primitive.
- [MTLCullMode](https://developer.apple.com/documentation/metal/mtlcullmode) — The mode that determines whether to perform culling and which type of primitive to cull.
- [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) — The geometric primitive type for drawing commands.
- [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) — The index type for an index buffer that references vertices of geometric primitives.
- [MTLDepthClipMode](https://developer.apple.com/documentation/metal/mtldepthclipmode) — The mode that determines how to deal with fragments outside of the near or far planes.
- [MTLVisibilityResultMode](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode) — The mode that determines what, if anything, the GPU writes to the results buffer, after the GPU executes the render pass.
