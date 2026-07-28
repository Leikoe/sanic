# MTLIndirectInstanceAccelerationStructureDescriptor

*Class · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor>

A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.

## Declaration

```swift
class MTLIndirectInstanceAccelerationStructureDescriptor
```

## Topics

### Instance Properties
- [instanceCountBuffer](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancecountbuffer)
- [instanceCountBufferOffset](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancecountbufferoffset)
- [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancedescriptorbuffer)
- [instanceDescriptorBufferOffset](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancedescriptorbufferoffset)
- [instanceDescriptorStride](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancedescriptorstride)
- [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancedescriptortype)
- [instanceTransformationMatrixLayout](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/instancetransformationmatrixlayout)
- [maxInstanceCount](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/maxinstancecount)
- [maxMotionTransformCount](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/maxmotiontransformcount)
- [motionTransformBuffer](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformbuffer)
- [motionTransformBufferOffset](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformbufferoffset) — The offset, in bytes, to the descripton of the first motion transform.
- [motionTransformCountBuffer](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformcountbuffer)
- [motionTransformCountBufferOffset](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformcountbufferoffset)
- [motionTransformStride](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformstride)
- [motionTransformType](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformtype)

## See also

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTLAccelerationStructureInstanceOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions) — Options for adjusting the behavior of an instanced acceleration structure.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
