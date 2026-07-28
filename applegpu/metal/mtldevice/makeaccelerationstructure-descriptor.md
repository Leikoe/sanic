# makeAccelerationStructure(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(descriptor:)>

Creates a new ray-tracing acceleration structure from a descriptor.

## Declaration

```swift
func makeAccelerationStructure(descriptor: MTLAccelerationStructureDescriptor) -> (any MTLAccelerationStructure)?
```

## Parameters

- **descriptor** — An [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) instance.

## Return Value

A new [MTLAccelerationStructure](https://developer.apple.com/documentation/metal/mtlaccelerationstructure) instance if the method completed successfully; otherwise `nil`.

## See also

### Creating acceleration structures for ray tracing
- [makeAccelerationStructure(size:)](https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(size:)) — Creates a new acceleration structure with a specific size.
- [accelerationStructureSizes(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/accelerationstructuresizes(descriptor:)) — Returns the buffer sizes the GPU device needs to build, refit, and store an acceleration structure.
- [MTLAccelerationStructureSizes](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes) — The expected sizes for a ray-tracing acceleration structure.
