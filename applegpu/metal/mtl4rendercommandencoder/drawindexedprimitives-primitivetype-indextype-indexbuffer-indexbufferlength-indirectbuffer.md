# drawIndexedPrimitives(primitiveType:indexType:indexBuffer:indexBufferLength:indirectBuffer:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indextype:indexbuffer:indexbufferlength:indirectbuffer:)>

Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices and indirect arguments.

## Declaration

```swift
func drawIndexedPrimitives(primitiveType: MTLPrimitiveType, indexType: MTLIndexType, indexBuffer: MTLGPUAddress, indexBufferLength: Int, indirectBuffer: MTLGPUAddress)
```

## Parameters

- **primitiveType** — A [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) representing how the command interprets vertex argument data.
- **indexType** — A [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) instance that represents the index format.
- **indexBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains `indexCount` indices of `indexType` format. You are responsible for ensuring this address is aligned to 2 bytes if the `indexType` format is [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16), and aligned to 4 bytes if the format is [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).
- **indexBufferLength** — An integer that represents the length of `indexBuffer`, in bytes. You are responsible for ensuring this this size is a multiple of 2 if the `indexType` format is [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16), and a multiple of 4 if the format is [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32). If this draw call causes Metal to read indices at or beyond the `indexBufferLength`, Metal continues to execute them assigning a value of `0` to the `vertex_id` attribute.
- **indirectBuffer** — GPUAddress of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLDrawIndexedPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments) structure. This address requires 4-byte alignment.

## Discussion

When you use this function, Metal reads the parameters to the draw command from an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance, allowing you to implement a GPU-driven workflow where a compute pipeline state determines the draw arguments.

Because this is an indexed draw call, Metal interprets the contents of the indirect buffer to match the layout of struct [MTLDrawIndexedPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments), which includes `indexStart` and `indexCount` members, denoting a range within the index buffer you provide in the `indexBuffer` parameter.

The range of indices within the `indexBuffer` form the primitives Metal draws.

Metal imposes some restrictions on the index buffer’s address, which needs to be 2- or 4-byte aligned, and its length in bytes, which needs to be a multiple of 2 or 4, depending on whether the format of the index is [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) or [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).

Similarly, you are responsible for ensuring the indirect buffer’s address has 4-byte alignment.

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the indirect buffer that the `indirectBuffer` parameter references, and of the index buffer the `indexBuffer` parameter references.

## See also

### Drawing with indexed vertices
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:)) — Encodes a draw command that renders an instance of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:basevertex:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices, starting with a custom vertex and instance.
