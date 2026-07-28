# MTLAccelerationStructureSizes

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes>

The expected sizes for a ray-tracing acceleration structure.

## Declaration

```swift
struct MTLAccelerationStructureSizes
```

## Topics

### Retrieving the sizes
- [accelerationStructureSize](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/accelerationstructuresize) — The size of the acceleration structure, in bytes.
- [buildScratchBufferSize](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/buildscratchbuffersize) — The amount of scratch memory, in bytes, the GPU devices needs to build the acceleration structure.
- [refitScratchBufferSize](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/refitscratchbuffersize) — The amount of scratch memory, in bytes, the GPU device needs to refit the acceleration structure.

### Creating an acceleration size structure
- [init()](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/init()) — Creates an acceleration sizes instance with default values.
- [init(accelerationStructureSize:buildScratchBufferSize:refitScratchBufferSize:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes/init(accelerationstructuresize:buildscratchbuffersize:refitscratchbuffersize:)) — Creates an acceleration sizes instance with specific values.

## See also

### Creating acceleration structures for ray tracing
- [makeAccelerationStructure(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(descriptor:)) — Creates a new ray-tracing acceleration structure from a descriptor.
- [makeAccelerationStructure(size:)](https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(size:)) — Creates a new acceleration structure with a specific size.
- [accelerationStructureSizes(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/accelerationstructuresizes(descriptor:)) — Returns the buffer sizes the GPU device needs to build, refit, and store an acceleration structure.
