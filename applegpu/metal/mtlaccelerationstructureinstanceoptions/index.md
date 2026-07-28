# MTLAccelerationStructureInstanceOptions

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions>

Options for adjusting the behavior of an instanced acceleration structure.

## Declaration

```swift
struct MTLAccelerationStructureInstanceOptions
```

## Topics

### Creating instance flags
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions/init(rawvalue:)) — Creates new usage options from a raw integer value.

### Usage options
- [disableTriangleCulling](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions/disabletriangleculling) — An option that turns off culling for this instance if ray intersector has culling enabled.
- [triangleFrontFacingWindingCounterClockwise](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions/trianglefrontfacingwindingcounterclockwise) — Specifies that the instance specifies front facing triangles in counter-clockwise order.
- [opaque](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions/opaque) — Specifies that intersectors should treat the instance as opaque.
- [nonOpaque](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstanceoptions/nonopaque) — Specifies that intersectors should treat the instance as non-opaque.

## See also

### Instance descriptors
- [MTLAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure.
- [MTLAccelerationStructureUserIDInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureuseridinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
- [MTLAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotioninstancedescriptor) — A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
- [MTL4IndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor) — Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
- [MTLIndirectInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
- [MTLIndirectAccelerationStructureInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructureinstancedescriptor) — A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
- [MTLIndirectAccelerationStructureMotionInstanceDescriptor](https://developer.apple.com/documentation/metal/mtlindirectaccelerationstructuremotioninstancedescriptor) — A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
