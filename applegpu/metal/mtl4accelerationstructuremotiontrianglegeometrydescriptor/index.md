# MTL4AccelerationStructureMotionTriangleGeometryDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor>

Describes motion triangle geometry, suitable for motion ray tracing.

## Declaration

```swift
class MTL4AccelerationStructureMotionTriangleGeometryDescriptor
```

## Overview

Use a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of all buffers this descriptor references when you build this acceleration structure.

## Topics

### Instance Properties
- [indexBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/indexbuffer) — Assigns an optional index buffer containing references to vertices in the vertex buffers you reference through the vertex buffers property.
- [indexType](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/indextype) — Specifies the size of the indices the `indexBuffer` contains, which is typically either 16 or 32-bits for each index.
- [transformationMatrixBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/transformationmatrixbuffer) — Assings an optional reference to a buffer containing a `float4x3` transformation matrix.
- [transformationMatrixLayout](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/transformationmatrixlayout) — Configures the layout for the transformation matrix in the transformation matrix buffer.
- [triangleCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/trianglecount) — Declares the number of triangles in the vertex buffers that the buffer in the vertex buffers property references.
- [vertexBuffers](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexbuffers) — Assigns a buffer where each entry contains a reference to a vertex buffer.
- [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexformat) — Defines the format of the vertices in the vertex buffers.
- [vertexStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor/vertexstride) — Sets the stride, in bytes, between vertices in all the vertex buffer.

## See also

### Motion geometry descriptors
- [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) — A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
- [MTL4AccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor) — Describes motion curve geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioncurvegeometrydescriptor)
- [MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor) — Describes motion bounding box geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
- [MTLMotionKeyframeData](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata) — Geometry data for a specific keyframe to use in a moving instance.
