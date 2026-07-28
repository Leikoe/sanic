# boundingBoxBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbuffer>

A buffer that contains an array of bounding box structures.

## Declaration

```swift
var boundingBoxBuffer: (any MTLBuffer)? { get set }
```

## Discussion

The buffer contains an array of [MTLAxisAlignedBoundingBox](https://developer.apple.com/documentation/metal/mtlaxisalignedboundingbox-c.struct) structures, one for each bounding box in the geometry.

## See also

### Specifying bounding boxes data
- [boundingBoxBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbufferoffset) — The offset, in bytes, to the first bounding box in the buffer.
- [boundingBoxStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxstride) — The stride, in bytes, between bounding boxes in the buffer.
