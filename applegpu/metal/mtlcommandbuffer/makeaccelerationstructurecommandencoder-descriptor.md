# makeAccelerationStructureCommandEncoder(descriptor:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder(descriptor:)>

Creates a ray-tracing acceleration structure command encoder from a descriptor.

## Declaration

```swift
func makeAccelerationStructureCommandEncoder(descriptor: MTLAccelerationStructurePassDescriptor) -> any MTLAccelerationStructureCommandEncoder
```

## Parameters

- **descriptor** — An [MTLAccelerationStructurePassDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructurepassdescriptor) instance that configures the [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) the method returns.

## Discussion

Use an [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) instance’s methods to set up a single ray-tracing pass.

## See also

### Creating acceleration structure encoders
- [makeAccelerationStructureCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder()) — Creates a ray-tracing acceleration structure command encoder that uses default settings.
