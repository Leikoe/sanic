# boundingBoxBufferOffset

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbufferoffset>

The offset, in bytes, to the first bounding box in the buffer.

## Declaration

```swift
var boundingBoxBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of [boundingBoxStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxstride). Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Specifying bounding boxes data
- [boundingBoxBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbuffer) — A buffer that contains an array of bounding box structures.
- [boundingBoxStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxstride) — The stride, in bytes, between bounding boxes in the buffer.
