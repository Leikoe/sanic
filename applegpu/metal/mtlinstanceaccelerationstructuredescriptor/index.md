# MTLInstanceAccelerationStructureDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor>

A description of an acceleration structure that derives from instances of primitive acceleration structures.

## Declaration

```swift
class MTLInstanceAccelerationStructureDescriptor
```

## Overview

Metal provides acceleration structures with a two-level hierarchy. The bottom layer consists of primitive acceleration structures, which instance acceleration structures in the top level reference.

## Topics

### Specifying the instance structures
- [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptortype) — The format of the instance data in the descriptor buffer.
- [instancedAccelerationStructures](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedaccelerationstructures) — The bottom-level acceleration structures that instances use in the instance acceleration structure .
- [MTLAccelerationStructureInstanceDescriptorType](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype) — Options for specifying different kinds of instance types.

### Specifying the list of instances
- [instanceCount](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancecount) — The number of instances in the instance descriptor buffer.
- [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbuffer) — A buffer that contains descriptions of each instance in the acceleration structure.
- [instanceDescriptorBufferOffset](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbufferoffset) — The offset, in bytes, to the descripton of the first instance.
- [instanceDescriptorStride](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorstride) — The stride, in bytes, between instance descriptions.

### Specifying motion data
- [motionTransformCount](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformcount) — The number of motion transforms in the motion transform buffer.
- [motionTransformBuffer](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformbuffer) — A buffer that contains descriptions of each motion transform in the acceleration structure.
- [motionTransformBufferOffset](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformbufferoffset) — The offset, in bytes, to the descripton of the first motion transform.

### Instance Properties
- [instanceTransformationMatrixLayout](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancetransformationmatrixlayout)
- [motionTransformStride](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformstride)
- [motionTransformType](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformtype)

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
