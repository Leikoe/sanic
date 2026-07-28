# MTLAccelerationStructureCurveGeometryDescriptor

*Class · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor>

A descriptor you configure with curve geometry for building acceleration structures.

## Declaration

```swift
class MTLAccelerationStructureCurveGeometryDescriptor
```

## Topics

### Instance Properties
- [controlPointBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointbuffer) — A buffer that contains curve control points.
- [controlPointBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointbufferoffset) — The offset, in bytes, to the control point data in the buffer.
- [controlPointCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointcount) — The number of control points in the control point buffer.
- [controlPointFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointformat) — The format of the control points in the buffer.
- [controlPointStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointstride) — The stride, in bytes, between control points in the buffer.
- [curveBasis](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/curvebasis) — The basis function for the curve geometry.
- [curveEndCaps](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/curveendcaps) — An end-cap type for the curves in the geometry.
- [curveType](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/curvetype) — A curve type for curves in the geometry.
- [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indexbuffer) — A buffer that contains references to control points in the control point buffer.
- [indexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indexbufferoffset) — The offset, in bytes, to the index data in the buffer.
- [indexType](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indextype) — The size of each index in the index buffer.
- [radiusBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/radiusbuffer) — A buffer that contains the curve radius for each control point.
- [radiusBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/radiusbufferoffset) — The offset, in bytes, to the radius data in the buffer.
- [radiusFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/radiusformat) — The format of each radius in the radius buffer.
- [radiusStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/radiusstride) — The stride, in bytes, between the radius elements in the radius buffer.
- [segmentControlPointCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/segmentcontrolpointcount) — The number of control points in each curve segment.
- [segmentCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/segmentcount) — The number of curve segments in each curve of the geometry.

## See also

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.
