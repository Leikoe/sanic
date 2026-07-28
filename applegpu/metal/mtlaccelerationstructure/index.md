# MTLAccelerationStructure

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructure>

A collection of model data for GPU-accelerated intersection of rays with the model.

## Declaration

```swift
protocol MTLAccelerationStructure : MTLResource
```

## Overview

To accelerate ray tracing, the device instance needs to reorganize your model data into an optimized data structure for intersection testing on that GPU. Create  [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) instances to contain your model data and reference them in compute and render  commands that execute ray-tracing operations.

You don’t define classes that implement this protocol. To create an acceleration structure, you create a descriptor instance and configure its properties with your model data. Then call the [makeAccelerationStructure(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(descriptor:)) method on the Metal device instance to create the instance and reserve memory for the structure. To populate the structure with the data, use an [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) to encode GPU commands.

Metal provides multiple descriptor classes, each describing a different type of model data. Choose the appropriate descriptor for each acceleration structure you want to make. Most often, you create an acceleration structure for each list of triangles or bounding boxes. Then collect related geometry structures into a primitive acceleration structure. Create instance acceleration structures when you need to reference instances of primitive acceleration structures at different locations within a scene.

The table below summarizes the descriptor classes:

| Descriptor class | Usage |
|---|---|
| [MTLAccelerationStructureTriangleGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuretrianglegeometrydescriptor) | Describes an acceleration structure for a list of triangles. |
| [MTLAccelerationStructureBoundingBoxGeometryDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor) | Describes an acceleration structure for a list of bounding boxes. |
| [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) | Describes an acceleration structure for a list of bounding-box or triangle acceleration structures, effectively creating a union of all of the underlying geometry. |
| [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) | Describes an acceleration structure for a list of instances of primitive acceleration structures. |

## Topics

### Reading the structure’s size
- [size](https://developer.apple.com/documentation/metal/mtlaccelerationstructure/size) — The size of the acceleration structure’s memory allocation, in bytes.

### Instance Properties
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlaccelerationstructure/gpuresourceid)

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
