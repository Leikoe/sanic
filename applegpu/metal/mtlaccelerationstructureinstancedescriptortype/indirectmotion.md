# MTLAccelerationStructureInstanceDescriptorType.indirectMotion

*Case · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirectmotion>

An option specifying that the instance contains motion data, and enables using an instance descriptor memory layout that the GPU can populate.

## Declaration

```swift
case indirectMotion
```

## Discussion

This instance type corresponds to the [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) memory layout.

## See also

### Specifying the instance descriptor type
- [MTLAccelerationStructureInstanceDescriptorType.default](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/default) — An option specifying that the instance uses the default characteristics.
- [MTLAccelerationStructureInstanceDescriptorType.userID](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/userid) — An option specifying that the instance contains a user identifier.
- [MTLAccelerationStructureInstanceDescriptorType.motion](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/motion) — An option specifying that the instance contains motion data.
- [MTLAccelerationStructureInstanceDescriptorType.indirect](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirect) — An option that enables an instance descriptor memory layout the GPU can populate.
