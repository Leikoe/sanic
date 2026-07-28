# indexType

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/indextype>

The data type of indices in the index buffer.

## Declaration

```swift
var indexType: MTLIndexType { get set }
```

## Discussion

The index type needs to be [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) or [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32). The default is [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).

## See also

### Configuring index data
- [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/indexbuffer) — A buffer that contains indices for the vertices that compose the triangle list.
- [indexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/indexbufferoffset) — The offset, in bytes, to the first index in the buffer.
