# MTLAccelerationStructureUserIDInstanceDescriptor

*Structure · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor>

A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.

## Declaration

```swift
struct MTLAccelerationStructureUserIDInstanceDescriptor
```

## Topics

### Creating an instance descriptor
- [init()](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/init()) — Creates a default acceleration structure instance.
- [init(transformationMatrix:options:mask:intersectionFunctionTableOffset:accelerationStructureIndex:userID:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/init(transformationmatrix:options:mask:intersectionfunctiontableoffset:accelerationstructureindex:userid:)) — Creates a new acceleration structure instance.

### Specifying the instance
- [accelerationStructureIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/accelerationstructureindex) — The index of the acceleration structure to use for the instance.

### Specifying the instance transform
- [transformationMatrix](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/transformationmatrix) — The transform for placing and orienting the instance in the scene.

### Customizing intersection and hit tests for the instance
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/intersectionfunctiontableoffset) — An offset for determining which function in the intersection function table Metal calls when testing a ray against the instance.
- [options](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/options) — The options for the instance.
- [mask](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/mask) — A mask to use for the instance when testing a ray against the geometry.

### Specifying the user identifier
- [userID](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor/userid) — The user identifier for the instance.

## See also

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
