# indexBufferOffset

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbufferoffset>

The offset, in bytes, to the first index in the buffer.

## Declaration

```swift
var indexBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of the index data type size and aligned to the index data type’s alignment. Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Specifying index data
- [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbuffer) — A buffer that contains indices for the vertices that compose the triangle list.
- [indexType](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indextype) — The data type of indices in the index buffer.
