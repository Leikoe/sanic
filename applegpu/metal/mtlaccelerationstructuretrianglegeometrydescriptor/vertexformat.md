# vertexFormat

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexformat>

The format of each vertex position in the vertex buffer property.

## Declaration

```swift
var vertexFormat: MTLAttributeFormat { get set }
```

## Discussion

Set this property to a value that represents the pixel format of the data you assign to the [vertexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbuffer) property. The property’s default is [MTLAttributeFormat.float3](https://developer.apple.com/documentation/metal/mtlattributeformat/float3).

## See also

### Configuring vertex data
- [vertexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbuffer) — A buffer that contains vertex data.
- [vertexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbufferoffset) — The offset, in bytes, for the first vertex in the vertex buffer.
- [vertexStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexstride) — The stride, in bytes, between vertices in the vertex buffer.
