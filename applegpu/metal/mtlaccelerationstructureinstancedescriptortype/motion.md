# MTLAccelerationStructureInstanceDescriptorType.motion

*Case · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/motion>

An option specifying that the instance contains motion data.

## Declaration

```swift
case motion
```

## Discussion

This instance type corresponds to the [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) structure memory layout.

## See also

### Specifying the instance descriptor type
- [MTLAccelerationStructureInstanceDescriptorType.default](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/default) — An option specifying that the instance uses the default characteristics.
- [MTLAccelerationStructureInstanceDescriptorType.userID](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/userid) — An option specifying that the instance contains a user identifier.
- [MTLAccelerationStructureInstanceDescriptorType.indirect](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirect) — An option that enables an instance descriptor memory layout the GPU can populate.
- [MTLAccelerationStructureInstanceDescriptorType.indirectMotion](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirectmotion) — An option specifying that the instance contains motion data, and enables using an instance descriptor memory layout that the GPU can populate.
