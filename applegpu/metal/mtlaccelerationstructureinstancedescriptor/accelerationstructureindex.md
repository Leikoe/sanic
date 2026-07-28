# accelerationStructureIndex

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/accelerationstructureindex>

The index of the acceleration structure to use for the instance.

## Declaration

```swift
var accelerationStructureIndex: UInt32
```

## Discussion

This index refers to a bottom-level instance in the [instancedAccelerationStructures](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures) of the [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) that you use to create the new instance acceleration structure.

## See also

### Related Documentation
- [instancedAccelerationStructures](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures) — The bottom-level acceleration structures that instances use in the instance acceleration structure .
