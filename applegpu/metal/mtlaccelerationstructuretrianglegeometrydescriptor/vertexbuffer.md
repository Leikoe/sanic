# vertexBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbuffer>

A buffer that contains vertex data.

## Declaration

```swift
var vertexBuffer: (any MTLBuffer)? { get set }
```

## Discussion

The [vertexFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexformat) property defines the format of each vertex position in the buffer. You need to set a vertex buffer before creating the acceleration structure.

## See also

### Configuring vertex data
- [vertexFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexformat) — The format of each vertex position in the vertex buffer property.
- [vertexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbufferoffset) — The offset, in bytes, for the first vertex in the vertex buffer.
- [vertexStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexstride) — The stride, in bytes, between vertices in the vertex buffer.
