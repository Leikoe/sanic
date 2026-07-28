# MTLAccelerationStructureInstanceDescriptorType

*Enumeration · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype>

Options for specifying different kinds of instance types.

## Declaration

```swift
enum MTLAccelerationStructureInstanceDescriptorType
```

## Topics

### Specifying the instance descriptor type
- [MTLAccelerationStructureInstanceDescriptorType.default](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/default) — An option specifying that the instance uses the default characteristics.
- [MTLAccelerationStructureInstanceDescriptorType.userID](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/userid) — An option specifying that the instance contains a user identifier.
- [MTLAccelerationStructureInstanceDescriptorType.motion](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/motion) — An option specifying that the instance contains motion data.
- [MTLAccelerationStructureInstanceDescriptorType.indirect](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirect) — An option that enables an instance descriptor memory layout the GPU can populate.
- [MTLAccelerationStructureInstanceDescriptorType.indirectMotion](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirectmotion) — An option specifying that the instance contains motion data, and enables using an instance descriptor memory layout that the GPU can populate.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/init(rawvalue:))

## See also

### Specifying the instance structures
- [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptortype) — The format of the instance data in the descriptor buffer.
- [instancedAccelerationStructures](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures) — The bottom-level acceleration structures that instances use in the instance acceleration structure .
