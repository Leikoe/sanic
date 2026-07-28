# MTLAccelerationStructureUsage

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage>

Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.

## Declaration

```swift
struct MTLAccelerationStructureUsage
```

## Topics

### Applying options
- [refit](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/refit) — An option that lets you update an acceleration structure after creating it.
- [preferFastBuild](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastbuild) — An option that instructs Metal to build an acceleration structure quickly.
- [preferFastIntersection](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/preferfastintersection) — An option that instructs Metal to prioritize building an acceleration structure with better intersection performance.
- [minimizeMemory](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/minimizememory) — An option that instructs Metal to prioritize building an acceleration structure that needs less memory.
- [extendedLimits](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/extendedlimits) — An option that increases an acceleration structure’s storage capacity.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/init(rawvalue:)) — Creates new usage options instance from a raw integer value.

## See also

### Acceleration structures
- [Improving ray-tracing data access using per-primitive data](https://developer.apple.com/documentation/metal/improving-ray-tracing-data-access-using-per-primitive-data) — Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
- [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) — A collection of model data for GPU-accelerated intersection of rays with the model.
- [MTL4AccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor) — Base class for Metal 4 acceleration structure descriptors.
- [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) — A base class for classes that define the configuration for a new acceleration structure.
- [MTL4PrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor) — Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
- [MTLPrimitiveAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor) — A description of an acceleration structure that contains geometry primitives.
- [MTL4InstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtl4instanceaccelerationstructuredescriptor) — Descriptor for an instance acceleration structure.
- [MTLInstanceAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor) — A description of an acceleration structure that derives from instances of primitive acceleration structures.
- [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) — Encodes commands that build and refit acceleration structures for a single pass.
- [MTLAccelerationStructureRefitOptions](https://developer.apple.com/documentation/metal/mtlaccelerationstructurerefitoptions)
