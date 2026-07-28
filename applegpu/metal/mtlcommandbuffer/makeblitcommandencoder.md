# makeBlitCommandEncoder()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder()>

Creates a block information transfer (blit) encoder.

## Declaration

```swift
func makeBlitCommandEncoder() -> (any MTLBlitCommandEncoder)?
```

## Discussion

Use an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) instance’s methods to create a block information transfer (blit) pass that quickly copies memory between a GPU device’s resources.

## See also

### Creating blit encoders
- [makeBlitCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder(descriptor:)) — Creates a block information transfer (blit) encoder from a descriptor.
