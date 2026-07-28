# MTLAccelerationStructureGeometryDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor>

A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.

## Declaration

```swift
class MTLAccelerationStructureGeometryDescriptor
```

## Overview

Don’t use this base class directly. Use one of the derived classes instead, as  [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) describes.

## Topics

### Specifying base geometry properties
- [label](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/label) — A label for the geometry structure, suitable for debugging.
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/intersectionfunctiontableoffset) — An index into the intersection table for determining which intersection function Metal calls when it intersects a ray with the acceleration structure.
- [opaque](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/opaque) — A Boolean value that determines whether the geometry data in the acceleration structure needs to skip triangle-intersection tests.
- [allowDuplicateIntersectionFunctionInvocation](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/allowduplicateintersectionfunctioninvocation) — A Boolean value that indicates whether Metal calls the ray-intersection test more than once per primitive on the structure.

### Instance Properties
- [primitiveDataBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/primitivedatabuffer)
- [primitiveDataBufferOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/primitivedatabufferoffset)
- [primitiveDataElementSize](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/primitivedataelementsize)
- [primitiveDataStride](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor/primitivedatastride)

## See also

### Related Documentation
- [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) — A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
- [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) — A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.

### Geometry descriptors
- [MTL4AccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor) — Base class for all Metal 4 acceleration structure geometry descriptors.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.
