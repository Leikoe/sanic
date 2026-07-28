# drawPrimitives(primitiveType:vertexStart:vertexCount:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:)>

Encodes a draw command that renders an instance of a geometric primitive.

## Declaration

```swift
func drawPrimitives(primitiveType: MTLPrimitiveType, vertexStart: Int, vertexCount: Int)
```

## Parameters

- **primitiveType** — A [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) representing how the command interprets vertex argument data.
- **vertexStart** — The lowest value the command passes to your vertex shader function’s parameter with the `[[vertex_id]]` attribute.
- **vertexCount** — An integer that represents the number of vertices of `primitiveType` the command draws.

## Discussion

This command assigns each vertex a unique `vertex_id` value that increases from `vertexStart` through `(vertexStart + vertexCount - 1)`.

Your vertex shader function can use this value to uniquely identify each vertex.

## See also

### Drawing with vertices
- [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive.
- [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive, starting with a custom instance identification number.
- [drawPrimitives(primitiveType:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:indirectbuffer:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indirect arguments.
