# vertexStride

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexstride>

The stride, in bytes, between vertices in the vertex buffer.

## Declaration

```swift
var vertexStride: Int { get set }
```

## Discussion

The stride needs to be at least 12 bytes and needs to be a multiple of 4 bytes. The default value is 12 bytes.

## See also

### Configuring vertex data
- [vertexFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexformat) — The format of each vertex position in the vertex buffer property.
- [vertexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbuffer) — A buffer that contains vertex data.
- [vertexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbufferoffset) — The offset, in bytes, for the first vertex in the vertex buffer.
