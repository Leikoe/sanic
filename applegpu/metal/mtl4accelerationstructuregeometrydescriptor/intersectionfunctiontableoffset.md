# intersectionFunctionTableOffset

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor/intersectionfunctiontableoffset>

Sets the offset that this geometry contributes to determining the intersection function to invoke when a ray intersects it.

## Declaration

```swift
var intersectionFunctionTableOffset: Int { get set }
```

## Discussion

When you perform a ray tracing operation in the Metal Shading Language, and provide the ray intersector object with an instance of [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable), Metal adds this offset to the instance offset from structs such as:

- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor)

- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor)

- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor)

- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor)

- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor)

The sum of these offsets provides an index into the intersection function table that the ray tracing system uses to retrieve and invoke the function at this index, allowing you to customize the intersection evaluation process.
