# drawPrimitives(primitiveType:indirectBuffer:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:indirectbuffer:)>

Encodes a draw command that renders multiple instances of a geometric primitive with indirect arguments.

## Declaration

```swift
func drawPrimitives(primitiveType: MTLPrimitiveType, indirectBuffer: MTLGPUAddress)
```

## Parameters

- **primitiveType** — A [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) representing how the command interprets vertex argument data.
- **indirectBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLDrawPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments) structure. You are responsible for ensuring that the alignment of this address is 4 bytes.

## Discussion

When you use this function, Metal reads the parameters to the draw command from an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance, allowing you to implement a GPU-driven workflow where a compute pipeline state determines the draw arguments.

You are responsible for ensuring that the address of the indirect buffer you provide to this method has 4-byte alignment.

Because this is a non-indexed draw call, Metal interprets the contents of the indirect buffer to match the layout of struct [MTLDrawPrimitivesIndirectArguments](https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments).

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the indirect buffer that the `indirectBuffer` parameter references.

## See also

### Drawing with vertices
- [drawPrimitives(primitiveType:vertexStart:vertexCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:)) — Encodes a draw command that renders an instance of a geometric primitive.
- [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive.
- [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive, starting with a custom instance identification number.
