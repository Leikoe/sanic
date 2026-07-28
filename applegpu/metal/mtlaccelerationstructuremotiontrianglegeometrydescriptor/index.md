# MTLAccelerationStructureMotionTriangleGeometryDescriptor

*Class · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor>

A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.

## Declaration

```swift
class MTLAccelerationStructureMotionTriangleGeometryDescriptor
```

## Topics

### Specifying the number of triangles
- [triangleCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/trianglecount) — The number of triangles in the buffers.

### Specifying index data
- [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbuffer) — A buffer that contains indices for the vertices that compose the triangle list.
- [indexType](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indextype) — The data type of indices in the index buffer.
- [indexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/indexbufferoffset) — The offset, in bytes, to the first index in the buffer.

### Specifying vertex data
- [vertexBuffers](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/vertexbuffers) — An array of motion keyframes, each containing triangle data.
- [vertexStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/vertexstride) — The stride, in bytes, between vertices in each vertex buffer.

### Instance Properties
- [transformationMatrixBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/transformationmatrixbuffer)
- [transformationMatrixBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/transformationmatrixbufferoffset)
- [transformationMatrixLayout](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/transformationmatrixlayout)
- [vertexFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor/vertexformat)

## See also

### Motion geometry descriptors
- [MTL4AccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor) — Describes motion triangle geometry, suitable for motion ray tracing.
- [MTL4AccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor) — Describes motion curve geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioncurvegeometrydescriptor)
- [MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor) — Describes motion bounding box geometry, suitable for motion ray tracing.
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
- [MTLMotionKeyframeData](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata) — Geometry data for a specific keyframe to use in a moving instance.
