# boundingBoxStride

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxstride>

The stride, in bytes, between bounding boxes in the buffer.

## Declaration

```swift
var boundingBoxStride: Int { get set }
```

## Discussion

The stride needs be at least 24 bytes, and be a multiple of 4 bytes. The default value is 24 bytes.

## See also

### Specifying bounding boxes data
- [boundingBoxBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbuffer) — A buffer that contains an array of bounding box structures.
- [boundingBoxBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbufferoffset) — The offset, in bytes, to the first bounding box in the buffer.
