# makeAccelerationStructureCommandEncoder()

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder()>

Creates a ray-tracing acceleration structure command encoder that uses default settings.

## Declaration

```swift
func makeAccelerationStructureCommandEncoder() -> (any MTLAccelerationStructureCommandEncoder)?
```

## Discussion

Use an [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder) instance’s methods to set up a single ray-tracing pass.

## See also

### Creating acceleration structure encoders
- [makeAccelerationStructureCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeaccelerationstructurecommandencoder(descriptor:)) — Creates a ray-tracing acceleration structure command encoder from a descriptor.
