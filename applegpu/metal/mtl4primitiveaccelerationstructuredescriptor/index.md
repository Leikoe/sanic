# MTL4PrimitiveAccelerationStructureDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor>

Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.

## Declaration

```swift
class MTL4PrimitiveAccelerationStructureDescriptor
```

## Topics

### Instance Properties
- [geometryDescriptors](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/geometrydescriptors) — Associates the array of geometry descriptors that comprise this primitive acceleration structure.
- [motionEndBorderMode](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionendbordermode) — Configures the motion border mode.
- [motionEndTime](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionendtime) — Configures the motion end time for this geometry.
- [motionKeyframeCount](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionkeyframecount) — Sets the motion keyframe count.
- [motionStartBorderMode](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionstartbordermode) — Configures the behavior when the ray-tracing system samples the acceleration structure before the motion start time.
- [motionStartTime](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor/motionstarttime) — Configures the motion start time for this geometry.

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureUsage](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage) — Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
