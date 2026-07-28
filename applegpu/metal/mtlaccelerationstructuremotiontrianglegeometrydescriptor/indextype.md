# indexType

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indextype>

The data type of indices in the index buffer.

## Declaration

```swift
var indexType: MTLIndexType { get set }
```

## Discussion

The index type needs to be [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) or [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32). The default is [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).

## See also

### Specifying index data
- [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbuffer) — A buffer that contains indices for the vertices that compose the triangle list.
- [indexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbufferoffset) — The offset, in bytes, to the first index in the buffer.
