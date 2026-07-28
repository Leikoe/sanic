# makeCommandBuffer()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()>

Returns a command buffer from the command queue that maintains strong references to resources.

## Declaration

```swift
func makeCommandBuffer() -> (any MTLCommandBuffer)?
```

## Discussion

The command buffers you create with this method maintain strong references to the resources you encode into it, including buffers, textures, samplers, and pipeline states. The command buffer releases these references after it finishes running on the GPU.

This method sets the [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbuffer/retainedreferences) property to [true](https://developer.apple.com/documentation/Swift/true) for the command buffer it creates.

Each command queue has a fixed number of command buffers for its lifetime (see [makeCommandQueue(maxCommandBufferCount:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:))). This method blocks the calling CPU thread when the queue doesn’t have any free command buffers, and returns after the GPU finishes executing one.

## See also

### Creating command buffers
- [makeCommandBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer(descriptor:)) — Returns a command buffer from the command queue that you configure with a descriptor.
- [makeCommandBufferWithUnretainedReferences()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbufferwithunretainedreferences()) — Returns a command buffer from the command queue that doesn’t maintain strong references to resources.
