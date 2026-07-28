# makeComputeCommandEncoder(dispatchType:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)>

Creates a compute command encoder with a dispatch type.

## Declaration

```swift
func makeComputeCommandEncoder(dispatchType: MTLDispatchType) -> (any MTLComputeCommandEncoder)?
```

## Parameters

- **dispatchType** — An [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) instance that indicates whether the compute pass the encoder creates runs commands serially or concurrently.

## Discussion

Use an [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance’s methods to set up a single compute pass.

## See also

### Creating compute encoders
- [makeComputeCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(descriptor:)) — Creates a compute command encoder from a descriptor.
- [makeComputeCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder()) — Creates a compute command encoder that uses default settings.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
