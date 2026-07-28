# MTLAccelerationStructureInstanceDescriptor

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor>

A description of an instance in an instanced geometry acceleration structure.

## Declaration

```swift
struct MTLAccelerationStructureInstanceDescriptor
```

## Topics

### Creating an instance descriptor
- [init()](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/init()) — Creates a default acceleration structure instance.
- [init(transformationMatrix:options:mask:intersectionFunctionTableOffset:accelerationStructureIndex:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/init(transformationmatrix:options:mask:intersectionfunctiontableoffset:accelerationstructureindex:)) — Creates a new acceleration structure instance.

### Specifying the instance
- [accelerationStructureIndex](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/accelerationstructureindex) — The index of the acceleration structure to use for the instance.

### Specifying the instance transform
- [transformationMatrix](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/transformationmatrix) — The transform for placing and orienting the instance in the scene.

### Customizing intersection and hit tests for the instance
- [intersectionFunctionTableOffset](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/intersectionfunctiontableoffset) — An offset for determining which function in the intersection function table Metal needs to call when testing a ray against the instance.
- [options](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/options) — The options for the instance.
- [mask](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor/mask) — A mask to use for the instance when testing a ray against the geometry.

## See also

### Instance descriptors
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
