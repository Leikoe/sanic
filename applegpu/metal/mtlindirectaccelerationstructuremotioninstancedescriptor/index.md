# MTLIndirectAccelerationStructureMotionInstanceDescriptor

*Structure · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor>

A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.

## Declaration

```swift
struct MTLIndirectAccelerationStructureMotionInstanceDescriptor
```

## Overview

This memory layout corresponds to the [MTLAccelerationStructureInstanceDescriptorType.indirectMotion](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirectmotion) instance type.

## Topics

### Specifying the instance
- [accelerationStructureID](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/accelerationstructureid) — The acceleration resource handle to use for this instance.

### Specifying motion data
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionstarttime) — The start time of the motion instance.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionstartbordermode) — The motion border mode describing what happens if Metal samples the acceleration structure before the motion start time.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionendtime) — The end time of the motion instance.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motionendbordermode) — The motion border mode describing what happens if Metal samples the acceleration structure after the motion end time.
- [motionTransformsCount](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motiontransformscount) — The number of motion transforms belonging to the motion instance.
- [motionTransformsStartIndex](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/motiontransformsstartindex) — The index of the first set of transforms describing one keyframe of the animation.

### Specifying the user identifier
- [userID](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/userid) — A user-assigned ID to help identify the instance.

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/init()) — Creates a default indirect acceleration structure instance.
- [init(options:mask:intersectionFunctionTableOffset:userID:accelerationStructureID:motionTransformsStartIndex:motionTransformsCount:motionStartBorderMode:motionEndBorderMode:motionStartTime:motionEndTime:)](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/init(options:mask:intersectionfunctiontableoffset:userid:accelerationstructureid:motiontransformsstartindex:motiontransformscount:motionstartbordermode:motionendbordermode:motionstarttime:motionendtime:)) — Creates an indirect acceleration structure instance.

### Instance Properties
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/intersectionfunctiontableoffset) — An offset for determining which function in the intersection function table Metal calls when testing a ray against the instance.
- [mask](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/mask) — An instance mask to ignore geometry during ray tracing.
- [options](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor/options) — The options for this instance.

## See also

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
