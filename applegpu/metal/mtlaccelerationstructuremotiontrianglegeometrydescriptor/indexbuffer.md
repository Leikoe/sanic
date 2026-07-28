# indexBuffer

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbuffer>

A buffer that contains indices for the vertices that compose the triangle list.

## Declaration

```swift
var indexBuffer: (any MTLBuffer)? { get set }
```

## Discussion

This property can be `nil`, in which case the vertex data defines the triangle list implicitly. You need to store indices in a packed data format.

## See also

### Specifying index data
- [indexType](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indextype) — The data type of indices in the index buffer.
- [indexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbufferoffset) — The offset, in bytes, to the first index in the buffer.
