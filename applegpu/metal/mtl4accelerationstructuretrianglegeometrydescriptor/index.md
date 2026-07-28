# MTL4AccelerationStructureTriangleGeometryDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor>

Describes triangle geometry suitable for ray tracing.

## Declaration

```swift
class MTL4AccelerationStructureTriangleGeometryDescriptor
```

## Overview

Use a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of all buffers this descriptor references when you build this acceleration structure.

## Topics

### Instance Properties
- [indexBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/indexbuffer) — Sets an optional index buffer containing references to vertices in the `vertexBuffer`.
- [indexType](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/indextype) — Configures the size of the indices the `indexBuffer` contains, which is typically either 16 or 32-bits for each index.
- [transformationMatrixBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/transformationmatrixbuffer) — Assigns an optional reference to a buffer containing a `float4x3` transformation matrix.
- [transformationMatrixLayout](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/transformationmatrixlayout) — Configures the layout for the transformation matrix in the transformation matrix buffer.
- [triangleCount](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/trianglecount) — Declares the number of triangles in this geometry descriptor.
- [vertexBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexbuffer) — Associates a vertex buffer containing triangle vertices.
- [vertexFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexformat) — Describes the format of the vertices in the vertex buffer.
- [vertexStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor/vertexstride) — Sets the stride, in bytes, between vertices in the vertex buffer.

## See also

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.
