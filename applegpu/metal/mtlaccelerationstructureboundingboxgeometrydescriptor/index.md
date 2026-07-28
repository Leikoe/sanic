# MTLAccelerationStructureBoundingBoxGeometryDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor>

A description of a list of bounding boxes to turn into an acceleration structure.

## Declaration

```swift
class MTLAccelerationStructureBoundingBoxGeometryDescriptor
```

## Topics

### Specifying the number of bounding boxes
- [boundingBoxCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxcount) — The number of bounding boxes in the bounding box buffer.

### Specifying bounding boxes data
- [boundingBoxBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbuffer) — A buffer that contains an array of bounding box structures.
- [boundingBoxBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxbufferoffset) — The offset, in bytes, to the first bounding box in the buffer.
- [boundingBoxStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor/boundingboxstride) — The stride, in bytes, between bounding boxes in the buffer.

## See also

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
