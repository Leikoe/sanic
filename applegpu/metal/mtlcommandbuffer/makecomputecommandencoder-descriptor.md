# makeComputeCommandEncoder(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(descriptor:)>

Creates a compute command encoder from a descriptor.

## Declaration

```swift
func makeComputeCommandEncoder(descriptor computePassDescriptor: MTLComputePassDescriptor) -> (any MTLComputeCommandEncoder)?
```

## Parameters

- **computePassDescriptor** — An [MTLComputePassDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor) instance that configures the [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) the method returns.

## Discussion

Use an [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance’s methods to set up a single compute pass.

## See also

### Creating compute encoders
- [makeComputeCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder()) — Creates a compute command encoder that uses default settings.
- [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)) — Creates a compute command encoder with a dispatch type.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
