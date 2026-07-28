# MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor>

Describes motion bounding box geometry, suitable for motion ray tracing.

## Declaration

```swift
class MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor
```

## Overview

You use bounding boxes to implement procedural geometry for ray tracing, such as spheres or any other shape you define by using intersection functions.

Use a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of all buffers this descriptor references when you build this acceleration structure.

## Topics

### Instance Properties
- [boundingBoxBuffers](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor/boundingboxbuffers) — Configures a reference to a buffer where each entry contains a reference to a buffer of bounding boxes.
- [boundingBoxCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor/boundingboxcount) — Declares the number of bounding boxes in each buffer that `boundingBoxBuffer` references.
- [boundingBoxStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor/boundingboxstride) — Declares the stride, in bytes, between bounding boxes in the bounding box buffers each entry in `boundingBoxBuffer` references.

## See also

### Motion geometry descriptors
- [MTL4AccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor) — Describes motion triangle geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) — A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
- [MTL4AccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor) — Describes motion curve geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioncurvegeometrydescriptor)
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
- [MTLMotionKeyframeData](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata) — Geometry data for a specific keyframe to use in a moving instance.
