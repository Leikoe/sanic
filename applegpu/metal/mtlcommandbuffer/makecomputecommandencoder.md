# makeComputeCommandEncoder()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder()>

Creates a compute command encoder that uses default settings.

## Declaration

```swift
func makeComputeCommandEncoder() -> (any MTLComputeCommandEncoder)?
```

## Discussion

Use an [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance’s methods to set up a single compute pass. The encoder this method returns dispatches its compute commands serially (see [MTLDispatchType.serial](https://developer.apple.com/documentation/metal/mtldispatchtype/serial)). To create a compute command encoder that dispatches commands concurrently (see [MTLDispatchType.concurrent](https://developer.apple.com/documentation/metal/mtldispatchtype/concurrent)), use the [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)) or [makeComputeCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(descriptor:)) method.

## See also

### Creating compute encoders
- [makeComputeCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(descriptor:)) — Creates a compute command encoder from a descriptor.
- [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)) — Creates a compute command encoder with a dispatch type.
- [MTLDispatchType](https://developer.apple.com/documentation/metal/mtldispatchtype) — The type of dispatch method to use when calling encoded functions.
