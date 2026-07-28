# drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:)>

Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices.

## Declaration

```swift
func drawIndexedPrimitives(primitiveType: MTLPrimitiveType, indexCount: Int, indexType: MTLIndexType, indexBuffer: MTLGPUAddress, indexBufferLength: Int, instanceCount: Int)
```

## Parameters

- **primitiveType** — A [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) representing how the command interprets vertex argument data.
- **indexCount** — An integer that represents the number of vertices the command reads from `indexBuffer`.
- **indexType** — A [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) instance that represents the index format.
- **indexBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains `indexCount` indices of `indexType` format. You are responsible for ensuring this address is aligned to 2 bytes if the `indexType` format is [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16), and aligned to 4 bytes if the format is [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).
- **indexBufferLength** — An integer that represents the length of `indexBuffer`, in bytes. You are responsible for ensuring this this size is a multiple of 2 if the `indexType` format is [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16), and a multiple of 4 if the format is [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32). Metal disregards this value and assigns `0` to the `vertex_id` attribute for all primitives that require loading indices at a byte offset of `indexBufferLength` or greater.
- **instanceCount** — An integer that represents the number of times the command draws `primitiveType` with `indexCount` vertices.

## Discussion

Use this method to perform instanced indexed drawing, where an index buffer determines how Metal assembles primitives.

The command assigns each drawing instance a unique `instance_id` value that increases from `0` through `(instanceCount - 1)`. Your shader can use this value to identify which instance the vertex belongs to.

Metal imposes some restrictions on the index buffer’s address, which needs to be 2- or 4-byte aligned, and its length in bytes, which needs to be a multiple of 2 or 4, depending on whether the format of the index is [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) or [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the index buffer the `indexBuffer` parameter references.

## See also

### Drawing with indexed vertices
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:)) — Encodes a draw command that renders an instance of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:basevertex:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices, starting with a custom vertex and instance.
- [drawIndexedPrimitives(primitiveType:indexType:indexBuffer:indexBufferLength:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indextype:indexbuffer:indexbufferlength:indirectbuffer:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices and indirect arguments.
