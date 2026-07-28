# vertexBufferOffset

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbufferoffset>

The offset, in bytes, for the first vertex in the vertex buffer.

## Declaration

```swift
var vertexBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of the vertex stride. Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions. The default value is `0`.

## See also

### Configuring vertex data
- [vertexFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexformat) — The format of each vertex position in the vertex buffer property.
- [vertexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbuffer) — A buffer that contains vertex data.
- [vertexStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexstride) — The stride, in bytes, between vertices in the vertex buffer.
