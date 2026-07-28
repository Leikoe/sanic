# drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:)>

Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices.

## Declaration

```swift
func drawIndexedPrimitives(type primitiveType: MTLPrimitiveType, indexCount: Int, indexType: MTLIndexType, indexBuffer: any MTLBuffer, indexBufferOffset: Int, instanceCount: Int)
```

## Parameters

- **primitiveType** — An [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) instance that represents how the command interprets vertex argument data. See the [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) method and its siblings for more information about setting an entry in the vertex shader argument table for buffers.
- **indexCount** — An integer that represents the number of vertices the command reads from `indexBuffer` for each instance.
- **indexType** — An [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) instance that represents the index’s format, including [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) and [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).
- **indexBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains the `indexCount` vertex indices of the `indexType` format.
- **indexBufferOffset** — An integer that represents the location that’s a multiple of the index size from the start of `indexBuffer` where the vertex indices begin.
- **instanceCount** — An integer that represents the number of times the command draws `primitiveType` with `indexCount` vertices.

## Discussion

You can complete a primitive and start a new one by passing a sentinel index value that’s the largest unsigned integer possible for `indexType`. For example, the largest unsigned integer for [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) and [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32) is `0xFFFF` and `0xFFFFFFFF`, respectively. The command finishes the current primitive and begins drawing a new one each time the command reads a sentinel index value.

The method records the encoder’s current rendering state and resources the command needs as it runs. You can safely change the encoder’s render pipeline state to encode other commands after calling this method. Subsequent changes to the state don’t affect the commands already in the encoder’s [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer).

## See also

### Drawing with indexed vertices
- [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:)) — Encodes a draw command that renders an instance of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:basevertex:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices, starting with a custom vertex and instance.
- [drawIndexedPrimitives(type:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indextype:indexbuffer:indexbufferoffset:indirectbuffer:indirectbufferoffset:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices and indirect arguments.
