# resourceStateCommandEncoder(with:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/resourcestatecommandencoder(with:)>

Creates a resource state command encoder from a descriptor.

## Declaration

```swift
func resourceStateCommandEncoder(with resourceStatePassDescriptor: MTLResourceStatePassDescriptor) -> (any MTLResourceStateCommandEncoder)?
```

## Parameters

- **resourceStatePassDescriptor** — An [MTLResourceStatePassDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepassdescriptor) instance that configures the [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) the method returns.

## Discussion

Use an [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) instance’s methods to create a pass that updates the state of one or more sparse textures.

## See also

### Creating resource state encoders
- [makeResourceStateCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeresourcestatecommandencoder()) — Creates a resource state command encoder that uses default settings.
