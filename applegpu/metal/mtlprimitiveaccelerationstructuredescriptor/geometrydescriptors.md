# geometryDescriptors

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/geometrydescriptors>

An array that contains the individual pieces of geometry that compose the acceleration structure.

## Declaration

```swift
var geometryDescriptors: [MTLAccelerationStructureGeometryDescriptor]? { get set }
```

## Discussion

The value of the [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount) property determines what kinds of geometry descriptors you can assign to this property and how you need to configure them.

If the value of [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount) is greater than 1, then the geometry descriptors need to be either [MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor) or [MTLAccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor) objects. Further, you need to provide exactly that many keyframes of data when creating those geometry descriptors. If [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount)  is 1, use [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) or [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) objects instead.

## See also

### Related Documentation
- [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount) — The number of keyframes in the geometry data.
