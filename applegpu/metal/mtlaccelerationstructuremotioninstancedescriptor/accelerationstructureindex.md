# accelerationStructureIndex

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/accelerationstructureindex>

The index of an acceleration structure which applies to the next acceleration-structure motion instance you create with the descriptor.

## Declaration

```swift
var accelerationStructureIndex: UInt32
```

## Discussion

This index refers to a bottom-level instance specified in the [instancedAccelerationStructures](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures) of the [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) used to create the new instance acceleration structure.

## See also

### Related Documentation
- [instancedAccelerationStructures](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures) — The bottom-level acceleration structures that instances use in the instance acceleration structure .
