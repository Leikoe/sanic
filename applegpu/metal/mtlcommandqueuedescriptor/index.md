# MTLCommandQueueDescriptor

*Class · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor>

A configuration that customizes the behavior for a new command queue.

## Declaration

```swift
class MTLCommandQueueDescriptor
```

## Topics

### Instance Properties
- [logState](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor/logstate) — The shader logging configuration that the command queue uses.
- [maxCommandBufferCount](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor/maxcommandbuffercount) — An integer that sets the maximum number of uncompleted command buffers the queue can allow.

## See also

### Submitting work to a GPU with Metal
- [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure) — Discover how Metal executes commands on a GPU.
- [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) — An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
- [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) — A container that stores a sequence of GPU commands that you encode into it.
- [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) — A configuration that customizes the behavior for a new command buffer.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) — An encoder that writes GPU commands into a command buffer.
