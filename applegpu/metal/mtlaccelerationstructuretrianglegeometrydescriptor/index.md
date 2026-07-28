# MTLAccelerationStructureTriangleGeometryDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor>

A description of a list of triangle primitives to turn into an acceleration structure.

## Declaration

```swift
class MTLAccelerationStructureTriangleGeometryDescriptor
```

## Topics

### Configuring the number of triangles
- [triangleCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/trianglecount) — The number of triangles in the buffers.

### Configuring index data
- [indexType](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/indextype) — The data type of indices in the index buffer.
- [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/indexbuffer) — A buffer that contains indices for the vertices that compose the triangle list.
- [indexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/indexbufferoffset) — The offset, in bytes, to the first index in the buffer.

### Configuring vertex data
- [vertexFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexformat) — The format of each vertex position in the vertex buffer property.
- [vertexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbuffer) — A buffer that contains vertex data.
- [vertexBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexbufferoffset) — The offset, in bytes, for the first vertex in the vertex buffer.
- [vertexStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/vertexstride) — The stride, in bytes, between vertices in the vertex buffer.

### Configuring transformation data
- [transformationMatrixLayout](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/transformationmatrixlayout)
- [transformationMatrixBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/transformationmatrixbuffer)
- [transformationMatrixBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor/transformationmatrixbufferoffset)

## See also

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.
