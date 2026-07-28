# MTL4AccelerationStructureGeometryDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor>

Base class for all Metal 4 acceleration structure geometry descriptors.

## Declaration

```swift
class MTL4AccelerationStructureGeometryDescriptor
```

## Overview

Don’t use this class directly. Use one of the derived classes instead.

## Topics

### Instance Properties
- [allowDuplicateIntersectionFunctionInvocation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/allowduplicateintersectionfunctioninvocation) — A boolean value that indicates whether the ray-tracing system in Metal allows the invocation of intersection functions more than once per ray-primitive intersection.
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/intersectionfunctiontableoffset) — Sets the offset that this geometry contributes to determining the intersection function to invoke when a ray intersects it.
- [label](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/label) — Assigns an optional label you can assign to this geometry for debugging purposes.
- [opaque](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/opaque) — Provides a hint to Metal that this geometry is opaque, potentially accelerating the ray/primitive intersection process.
- [primitiveDataBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedatabuffer) — Assigns optional buffer containing data to associate with each primitive in this geometry.
- [primitiveDataElementSize](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedataelementsize) — Sets the size, in bytes, of the data for each primitive in the primitive data buffer [primitiveDataBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedatabuffer) references.
- [primitiveDataStride](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedatastride) — Defines the stride, in bytes, between each primitive’s data in the primitive data buffer [primitiveDataBuffer](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/primitivedatabuffer) references.

## See also

### Geometry descriptors
- [MTLAccelerationStructureGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor) — A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
- [MTL4AccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor) — Describes triangle geometry suitable for ray tracing.
- [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) — A description of a list of triangle primitives to turn into an acceleration structure.
- [MTL4AccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor) — Describes curve geometry suitable for ray tracing.
- [MTLAccelerationStructureCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor) — A descriptor you configure with curve geometry for building acceleration structures.
- [MTLCurveType](https://developer.apple.com/documentation/metal/mtlcurvetype)
- [MTLCurveBasis](https://developer.apple.com/documentation/metal/mtlcurvebasis)
- [MTLCurveEndCaps](https://developer.apple.com/documentation/metal/mtlcurveendcaps)
- [MTL4AccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor) — Describes bounding-box geometry suitable for ray tracing.
- [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) — A description of a list of bounding boxes to turn into an acceleration structure.
