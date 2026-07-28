# MTLCommandBufferDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor>

A configuration that customizes the behavior for a new command buffer.

## Declaration

```swift
class MTLCommandBufferDescriptor
```

## Overview

Create a command buffer with a custom configuration by creating an [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) instance and passing it to an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance’s [makeCommandBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer(descriptor:)) method. You can configure whether the command buffer retains references to resources that its commands refer to with the [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) property. The command buffer can save extra error information, which is useful during development, by setting its [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) property to [encoderExecutionStatus](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption/encoderexecutionstatus).

## Topics

### Configuring the command buffer
- [logState](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/logstate) — The shader logging configuration that the command buffer uses.
- [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/retainedreferences) — A Boolean value that indicates whether the command buffer the descriptor creates maintains strong references to the resources it uses.
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/erroroptions) — The reporting configuration that indicates which information the GPU driver stores in a command buffer’s error property.
- [MTLCommandBufferErrorOption](https://developer.apple.com/documentation/metal/mtlcommandbuffererroroption) — Options for reporting errors from a command buffer.

## See also

### Submitting work to a GPU with Metal
- [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure) — Discover how Metal executes commands on a GPU.
- [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) — An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
- [MTLCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor) — A configuration that customizes the behavior for a new command queue.
- [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) — A container that stores a sequence of GPU commands that you encode into it.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) — An encoder that writes GPU commands into a command buffer.
