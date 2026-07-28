# MTL4AccelerationStructureCurveGeometryDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor>

Describes curve geometry suitable for ray tracing.

## Declaration

```swift
class MTL4AccelerationStructureCurveGeometryDescriptor
```

## Overview

Use a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of all buffers this descriptor references when you build this acceleration structure.

## Topics

### Instance Properties
- [controlPointBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/controlpointbuffer) — References a buffer containing curve control points.
- [controlPointCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/controlpointcount) — Declares the number of control points in the control point buffer.
- [controlPointFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/controlpointformat) — Declares the format of the control points the control point buffer references.
- [controlPointStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/controlpointstride) — Sets the stride, in bytes, between control points in the control point buffer the control point buffer references.
- [curveBasis](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/curvebasis) — Controls the curve basis function, determining how Metal interpolates the control points.
- [curveEndCaps](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/curveendcaps) — Sets the type of curve end caps.
- [curveType](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/curvetype) — Controls the curve type.
- [indexBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/indexbuffer) — Assigns an optional index buffer containing references to control points in the control point buffer.
- [indexType](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/indextype) — Specifies the size of the indices the `indexBuffer` contains, which is typically either 16 or 32-bits for each index.
- [radiusBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/radiusbuffer) — Assigns a reference to a buffer containing the curve radius for each control point.
- [radiusFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/radiusformat) — Declares the format of the radii in the radius buffer.
- [radiusStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/radiusstride) — Configures the stride, in bytes, between radii in the radius buffer.
- [segmentControlPointCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/segmentcontrolpointcount) — Declares the number of control points per curve segment.
- [segmentCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/segmentcount) — Declares the number of curve segments.

## See also

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.
