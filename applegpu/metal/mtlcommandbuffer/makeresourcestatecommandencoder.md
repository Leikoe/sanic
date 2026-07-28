# makeResourceStateCommandEncoder()

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeresourcestatecommandencoder()>

Creates a resource state command encoder that uses default settings.

## Declaration

```swift
func makeResourceStateCommandEncoder() -> (any MTLResourceStateCommandEncoder)?
```

## Discussion

Use an [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) instance’s methods to create a pass that updates the state of one or more sparse textures.

## See also

### Creating resource state encoders
- [resourceStateCommandEncoder(with:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/resourcestatecommandencoder(with:)) — Creates a resource state command encoder from a descriptor.
