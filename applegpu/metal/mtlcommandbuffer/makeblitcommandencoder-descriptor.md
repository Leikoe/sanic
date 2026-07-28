# makeBlitCommandEncoder(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder(descriptor:)>

Creates a block information transfer (blit) encoder from a descriptor.

## Declaration

```swift
func makeBlitCommandEncoder(descriptor blitPassDescriptor: MTLBlitPassDescriptor) -> (any MTLBlitCommandEncoder)?
```

## Parameters

- **blitPassDescriptor** — An [MTLBlitPassDescriptor](https://developer.apple.com/documentation/metal/mtlblitpassdescriptor) instance that configures the [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) the method returns.

## Discussion

Use an [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) instance’s methods to create a block information transfer (blit) pass that quickly copies memory between a GPU device’s resources.

## See also

### Creating blit encoders
- [makeBlitCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder()) — Creates a block information transfer (blit) encoder.
