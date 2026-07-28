# MTL4InstanceAccelerationStructureDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor>

Descriptor for an instance acceleration structure.

## Declaration

```swift
class MTL4InstanceAccelerationStructureDescriptor
```

## Overview

An instance acceleration structure references other acceleration structures, and provides the ability to “instantiate” them multiple times, each one with potentially a different transformation matrix.

You specify the properties of the instances in the acceleration structure this descriptor builds by providing a buffer of `structs` via its [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancedescriptorbuffer) property.

Use a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of all buffers and acceleration structures this descriptor references when you build this acceleration structure.

## Topics

### Instance Properties
- [instanceCount](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancecount) — Controls the number of instance descriptors in the instance descriptor buffer references.
- [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancedescriptorbuffer) — Assigns a reference to a buffer containing instance descriptors for acceleration structures to reference.
- [instanceDescriptorStride](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancedescriptorstride) — Sets the stride, in bytes, between instance descriptors the instance descriptor buffer references.
- [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancedescriptortype) — The type of instance descriptor that the instance descriptor buffer references.
- [instanceTransformationMatrixLayout](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/instancetransformationmatrixlayout) — Specifies the layout for the transformation matrices in the instance descriptor buffer and the motion transformation matrix buffer.
- [motionTransformBuffer](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/motiontransformbuffer) — A buffer containing transformation information for instance motion keyframes, formatted according to the motion transform type.
- [motionTransformCount](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/motiontransformcount) — Controls the total number of motion transforms in the motion transform buffer.
- [motionTransformStride](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/motiontransformstride) — Specify the stride for motion transform.
- [motionTransformType](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor/motiontransformtype) — Controls the type of motion transforms, either as a matrix or individual components.

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
