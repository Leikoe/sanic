# instanceDescriptorType

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancedescriptortype>

The type of instance descriptor that the instance descriptor buffer references.

## Declaration

```swift
var instanceDescriptorType: MTLAccelerationStructureInstanceDescriptorType { get set }
```

## Discussion

This value determines the layout Metal expects for the structs the instance descriptor buffer contains:

- [MTLAccelerationStructureInstanceDescriptorType.indirect](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirect): Use the [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) struct layout.

- [MTLAccelerationStructureInstanceDescriptorType.indirectMotion](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirectmotion): Use the [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) struct layout.

The default value is [MTLAccelerationStructureInstanceDescriptorType.indirect](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirect).
