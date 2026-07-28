# instancedAccelerationStructures

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures>

The bottom-level acceleration structures that instances use in the instance acceleration structure .

## Declaration

```swift
var instancedAccelerationStructures: [any MTLAccelerationStructure]? { get set }
```

## Discussion

Each instance in the instance descriptor buffer has an index into this array, specifying which acceleration structure to use for that instance.

## See also

### Related Documentation
- [accelerationStructureIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/accelerationstructureindex) — The index of the acceleration structure to use for the instance.

### Specifying the instance structures
- [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptortype) — The format of the instance data in the descriptor buffer.
- [MTLAccelerationStructureInstanceDescriptorType](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype) — Options for specifying different kinds of instance types.
