# MTLDrawIndexedPrimitivesIndirectArguments

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments>

The data layout required for drawing indexed primitives via indirect buffer calls.

## Declaration

```swift
struct MTLDrawIndexedPrimitivesIndirectArguments
```

## Overview

See also the [drawIndexedPrimitives(type:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indextype:indexbuffer:indexbufferoffset:indirectbuffer:indirectbufferoffset:)) method.

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/init()) — Returns a new data layout for drawing indexed primitives via indirect buffer calls.
- [init(indexCount:instanceCount:indexStart:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/init(indexcount:instancecount:indexstart:basevertex:baseinstance:)) — Returns a new data layout for drawing indexed primitives via indirect buffer calls, with specified parameters.

### Instance Properties
- [baseInstance](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/baseinstance) — The first instance to draw.
- [baseVertex](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/basevertex) — The first vertex to draw.
- [indexCount](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/indexcount) — For each instance, the number of indices to read from the index buffer.
- [indexStart](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/indexstart) — The first index to draw.
- [instanceCount](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments/instancecount) — The number of instances to draw.

## See also

### Render compute commands
- [MTLIndirectRenderCommand](https://developer.apple.com/documentation/metal/mtlindirectrendercommand) — A render command in an indirect command buffer.
- [MTLDrawPatchIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments) — The data layout required for drawing patches via indirect buffer calls.
- [MTLDrawPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments) — The data layout required for drawing primitives via indirect buffer calls.
