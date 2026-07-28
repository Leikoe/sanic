# MTLDrawPrimitivesIndirectArguments

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments>

The data layout required for drawing primitives via indirect buffer calls.

## Declaration

```swift
struct MTLDrawPrimitivesIndirectArguments
```

## Overview

See also the [drawPrimitives(type:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:indirectbuffer:indirectbufferoffset:)) method.

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments/init()) — Returns a new data layout for drawing primitives via indirect buffer calls.
- [init(vertexCount:instanceCount:vertexStart:baseInstance:)](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments/init(vertexcount:instancecount:vertexstart:baseinstance:)) — Returns a new data layout for drawing primitives via indirect buffer calls, with specified parameters.

### Instance Properties
- [baseInstance](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments/baseinstance) — The first instance to draw.
- [instanceCount](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments/instancecount) — The number of instances to draw.
- [vertexCount](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments/vertexcount) — The number of vertices to draw.
- [vertexStart](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments/vertexstart) — The first vertex to draw.

## See also

### Render compute commands
- [MTLIndirectRenderCommand](https://developer.apple.com/documentation/metal/mtlindirectrendercommand) — A render command in an indirect command buffer.
- [MTLDrawPatchIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawpatchindirectarguments) — The data layout required for drawing patches via indirect buffer calls.
- [MTLDrawIndexedPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments) — The data layout required for drawing indexed primitives via indirect buffer calls.
