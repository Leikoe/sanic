# MTLIndirectAccelerationStructureInstanceDescriptor

*Structure · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor>

A description of an instance in an instanced geometry acceleration structure that the GPU can populate.

## Declaration

```swift
struct MTLIndirectAccelerationStructureInstanceDescriptor
```

## Overview

This memory layout corresponds to the [MTLAccelerationStructureInstanceDescriptorType.indirect](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype/indirect) instance type.

## Topics

### Initializers
- [init()](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/init())
- [init(transformationMatrix:options:mask:intersectionFunctionTableOffset:userID:accelerationStructureID:)](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/init(transformationmatrix:options:mask:intersectionfunctiontableoffset:userid:accelerationstructureid:))

### Instance Properties
- [accelerationStructureID](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/accelerationstructureid)
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/intersectionfunctiontableoffset)
- [mask](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/mask)
- [options](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/options)
- [transformationMatrix](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/transformationmatrix)
- [userID](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor/userid)

## See also

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
