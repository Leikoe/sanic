# MTLMotionKeyframeData

*Class · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlmotionkeyframedata>

Geometry data for a specific keyframe to use in a moving instance.

## Declaration

```swift
class MTLMotionKeyframeData
```

## Overview

An [MTLMotionKeyframeData](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata) instance describes the location of geometry data for a keyframe. The exact type of data can vary, depending on which kind of motion descriptor you create. For an [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) instance, the buffer data is a list of bounding boxes. For an [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor), the buffer data is a list of vertices.

## Topics

### Specifying the keyframe data
- [buffer](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata/buffer) — The buffer that holds the geometry data.
- [offset](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata/offset) — The offset, in bytes, to the keyframe data.

## See also

### Motion geometry descriptors
- [MTL4AccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor) — Describes motion triangle geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) — A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
- [MTL4AccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor) — Describes motion curve geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioncurvegeometrydescriptor)
- [MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor) — Describes motion bounding box geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
