# MTLCommandQueue

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue>

An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.

## Declaration

```swift
protocol MTLCommandQueue : NSObjectProtocol, Sendable
```

## Overview

A command queue maintains an ordered list of command buffers. You use a command queue to:

- Create command buffers, which you fill with commands for the GPU device that creates the queue

- Submit command buffers to run on that GPU

Create a command queue from an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance by calling its [makeCommandQueue()](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue()) or [makeCommandQueue(maxCommandBufferCount:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:)) method. Typically, you create one or more command queues when your app launches and then keep them throughout your app’s lifetime.

With each [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance you create, you can create [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instances for that queue by calling its [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()) or [makeCommandBufferWithUnretainedReferences()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbufferwithunretainedreferences()) method.

> **Note:**
>  Each command queue is thread-safe and allows you to encode commands in multiple command buffers simultaneously.

For more information about command buffers and encoding GPU commands to them — such as rendering images and computing data in parallel — see [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure).

## Topics

### Creating command buffers
- [makeCommandBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer(descriptor:)) — Returns a command buffer from the command queue that you configure with a descriptor.
- [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()) — Returns a command buffer from the command queue that maintains strong references to resources.
- [makeCommandBufferWithUnretainedReferences()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbufferwithunretainedreferences()) — Returns a command buffer from the command queue that doesn’t maintain strong references to resources.

### Attaching residency sets
- [addResidencySet(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/addresidencyset(_:)) — Applies a residency set to a queue, which Metal applies to the queue’s command buffers as you commit them.
- [addResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/addresidencysets(_:)) — Applies multiple residency sets to a queue, which Metal applies to the queue’s command buffers as you commit them.

### Detaching residency sets
- [removeResidencySet(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/removeresidencyset(_:)) — Removes a residency set from a command queue’s list, which means Metal doesn’t apply it to the queue’s command buffers as you commit them.
- [removeResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/removeresidencysets(_:)) — Removes multiple residency sets from a command queue’s list, which means Metal doesn’t apply them to the queue’s command buffers as you commit them.

### Identifying the command queue
- [device](https://developer.apple.com/documentation/metal/mtlcommandqueue/device) — The GPU device that creates the command queue.
- [label](https://developer.apple.com/documentation/metal/mtlcommandqueue/label) — An optional name that can help you identify the command queue.

### Deprecated
- [insertDebugCaptureBoundary()](https://developer.apple.com/documentation/metal/mtlcommandqueue/insertdebugcaptureboundary()) — Informs Xcode about when GPU Frame Capture starts and stops.

## See also

### Submitting work to a GPU with Metal
- [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure) — Discover how Metal executes commands on a GPU.
- [MTLCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor) — A configuration that customizes the behavior for a new command queue.
- [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) — A container that stores a sequence of GPU commands that you encode into it.
- [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) — A configuration that customizes the behavior for a new command buffer.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) — An encoder that writes GPU commands into a command buffer.
