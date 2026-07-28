# makeCommandBuffer(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer(descriptor:)>

Returns a command buffer from the command queue that you configure with a descriptor.

## Declaration

```swift
func makeCommandBuffer(descriptor: MTLCommandBufferDescriptor) -> (any MTLCommandBuffer)?
```

## Parameters

- **descriptor** — An [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) instance that configures the [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) the method returns.

## Discussion

Use this method to create a command buffer that you configure with a descriptor. You can configure whether the command buffer retains references to resources that its commands refer to by setting the `descriptor` parameter’s [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) property. You can also configure whether the command buffer saves extra error information, which is useful during development, by setting the descriptor’s [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) property.

Each command queue has a fixed number of command buffers for its lifetime (see [makeCommandQueue(maxCommandBufferCount:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:))). This method blocks the calling CPU thread when the queue doesn’t have any free command buffers, and returns after the GPU finishes executing one.

## See also

### Creating command buffers
- [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()) — Returns a command buffer from the command queue that maintains strong references to resources.
- [makeCommandBufferWithUnretainedReferences()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbufferwithunretainedreferences()) — Returns a command buffer from the command queue that doesn’t maintain strong references to resources.
