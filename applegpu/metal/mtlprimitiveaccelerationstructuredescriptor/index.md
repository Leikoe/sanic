# MTLPrimitiveAccelerationStructureDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor>

A description of an acceleration structure that contains geometry primitives.

## Declaration

```swift
class MTLPrimitiveAccelerationStructureDescriptor
```

## Overview

Metal provides acceleration structures with a two-level hierarchy. The bottom layer consists of primitive acceleration structures, which instance acceleration structures in the top level reference.

## Topics

### Specifying geometry
- [geometryDescriptors](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/geometrydescriptors) — An array that contains the individual pieces of geometry that compose the acceleration structure.

### Specifying motion behavior
- [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionkeyframecount) — The number of keyframes in the geometry data.
- [motionStartTime](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstarttime) — The start time for the range of motion that the keyframe data describes.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendtime) — The end time for the range of motion that the keyframe data describes.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionstartbordermode) — The mode to use when handling timestamps before the start time.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor/motionendbordermode) — The mode to use when handling timestamps after the end time.
- [MTLMotionBorderMode](https://developer.apple.com/documentation/metal/mtlmotionbordermode) — Options for specifying how the acceleration structure handles timestamps that are outside the specified range.

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
