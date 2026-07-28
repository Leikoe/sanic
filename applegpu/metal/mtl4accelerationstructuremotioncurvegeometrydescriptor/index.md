# MTL4AccelerationStructureMotionCurveGeometryDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor>

Describes motion curve geometry, suitable for motion ray tracing.

## Declaration

```swift
class MTL4AccelerationStructureMotionCurveGeometryDescriptor
```

## Overview

Use a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of all buffers this descriptor references when you build this acceleration structure.

## Topics

### Instance Properties
- [controlPointBuffers](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/controlpointbuffers) — Assigns a reference to a buffer where each entry contains a reference to a buffer of control points.
- [controlPointCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/controlpointcount) — Specifies the number of control points in the buffers the control point buffers reference.
- [controlPointFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/controlpointformat) — Declares the format of the control points in the buffers that the control point buffers reference.
- [controlPointStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/controlpointstride) — Sets the stride, in bytes, between control points in the control point buffer.
- [curveBasis](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/curvebasis) — Sets the curve basis function, determining how Metal interpolates the control points.
- [curveEndCaps](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/curveendcaps) — Configures the type of curve end caps.
- [curveType](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/curvetype) — Controls the curve type.
- [indexBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/indexbuffer) — Assigns an optional index buffer containing references to control points in the control point buffers.
- [indexType](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/indextype) — Configures the size of the indices the `indexBuffer` contains, which is typically either 16 or 32-bits for each index.
- [radiusBuffers](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusbuffers) — Assigns a reference to a buffer containing, in turn, references to curve radii buffers.
- [radiusFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusformat) — Sets the format of the radii in the radius buffer.
- [radiusStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusstride) — Sets the stride, in bytes, between radii in the radius buffer.
- [segmentControlPointCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/segmentcontrolpointcount) — Controls the number of control points per curve segment.
- [segmentCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/segmentcount) — Declares the number of curve segments.

## See also

### Motion geometry descriptors
- [MTL4AccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor) — Describes motion triangle geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) — A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
- [MTLAccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioncurvegeometrydescriptor)
- [MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor) — Describes motion bounding box geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
- [MTLMotionKeyframeData](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata) — Geometry data for a specific keyframe to use in a moving instance.
