# MTLAccelerationStructureMotionInstanceDescriptor

*Structure · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor>

A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.

## Declaration

```swift
struct MTLAccelerationStructureMotionInstanceDescriptor
```

## Topics

### Creating an instance descriptor
- [init()](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/init()) — Creates an acceleration-structure motion instance with default property values.
- [init(options:mask:intersectionFunctionTableOffset:accelerationStructureIndex:userID:motionTransformsStartIndex:motionTransformsCount:motionStartBorderMode:motionEndBorderMode:motionStartTime:motionEndTime:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/init(options:mask:intersectionfunctiontableoffset:accelerationstructureindex:userid:motiontransformsstartindex:motiontransformscount:motionstartbordermode:motionendbordermode:motionstarttime:motionendtime:)) — Creates an acceleration-structure motion instance with the property values you provide.

### Specifying the instance
- [accelerationStructureIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/accelerationstructureindex) — The index of an acceleration structure which applies to the next acceleration-structure motion instance you create with the descriptor.

### Specifying motion data
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionstarttime) — A starting time for the range of motion that the key-frame data represents.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionendtime) — An ending time for the range of motion that the key-frame data represents.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionstartbordermode) — A behavior that configures how a motion instance handles timestamps before a starting time.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motionendbordermode) — A behavior that configures how a motion instance handles timestamps after an ending time.
- [motionTransformsStartIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motiontransformsstartindex) — The index of motion data that represents the first key-frame motion data, which applies to the next acceleration-structure motion instance you create with the descriptor.
- [motionTransformsCount](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/motiontransformscount) — The number of motion data key-frames, which applies to the next acceleration-structure motion instance you create with the descriptor.

### Customizing intersection and hit tests for the instance
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/intersectionfunctiontableoffset) — An offset into the intersection-function table for ray tracing, which applies to the next acceleration-structure motion instance you create with the descriptor.
- [options](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/options) — An option set which applies to the next acceleration structure motion-instance you create with the descriptor.
- [mask](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/mask) — A mask for testing ray-tracing rays with a scene’s geometry, which applies to the next acceleration-structure motion instance you create with the descriptor.

### Specifying the user identifier
- [userID](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor/userid) — An unique identifier, which applies to the next acceleration-structure motion instance you create with the descriptor.

## See also

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
