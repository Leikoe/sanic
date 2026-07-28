# geometryDescriptors

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/geometrydescriptors>

Associates the array of geometry descriptors that comprise this primitive acceleration structure.

## Declaration

```swift
var geometryDescriptors: [MTL4AccelerationStructureGeometryDescriptor]? { get set }
```

## Discussion

If you enable keyframe motion by setting property [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionkeyframecount) to a value greater than `1`, then all geometry descriptors this array references need to be motion geometry descriptors and have a number of primitive buffers equals to [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionkeyframecount).

Example of motion geometry descriptors include: [MTL4AccelerationStructureMotionTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor), [MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor), [MTL4AccelerationStructureMotionCurveGeometryDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor).
