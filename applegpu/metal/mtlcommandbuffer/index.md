# MTLCommandBuffer

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer>

A container that stores a sequence of GPU commands that you encode into it.

## Declaration

```swift
protocol MTLCommandBuffer : NSObjectProtocol
```

## Overview

A command buffer represents a chunk of work for the GPU that stores the commands you encode to it, as well as any resources those commands need. You primarily use a command buffer to:

- Create command encoders and call their methods to add commands to the buffer

- Optionally reserve a place for the command buffer in its command queue by *enqueuing* the command buffer, even before you encode any commands into it

- Submit, or commit_,_ the contents of the command buffer to the command queue that creates it to run on the GPU device the queue represents

Create a command encoder from an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance by calling its [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()) method. Typically, you create one or more command queues when your app launches and then keep them throughout your app’s lifetime.

To add commands to an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance, create an encoder from one of its factory methods, including:

- An [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance by calling [makeRenderCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makerendercommandencoder(descriptor:))

- An [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance by calling [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:))

- An [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) instance by calling [makeBlitCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder()) or [makeBlitCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder(descriptor:))

- An [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) instance by calling [makeParallelRenderCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeparallelrendercommandencoder(descriptor:))

> **Note:**
>  All encoders inherit additional methods from the [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder).

You can use only a single encoder at a time to add commands to a command buffer. To start using a different command encoder, first signal that you’re done with the current encoder by calling its [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method. Then create another command encoder from the command buffer and continue adding commands to the buffer with the new encoder.

Repeat the process until you finish encoding commands to the command buffer and are ready to run the buffer’s contents on the GPU. Then submit the command buffer to the command queue that you used to create it by calling the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method. After an app commits a command buffer, you check its [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) property or block a thread by calling its [waitUntilScheduled()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilscheduled()) or [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) methods.

You also have the option to reserve a place for the command buffer in its command queue by calling the command buffer’s [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) method. You can call this method exactly once at any time before you commit the buffer to the queue. If you don’t enqueue a command buffer, it implicitly enqueues itself when you commit it. Each command queue ensures the order that you enqueue its command buffers is the same order the queue schedules them to run on the GPU.

> **Tip:**
>  Establish an order of execution for multiple command buffers you encode in parallel by first calling their [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) methods in that order.

For example, a multithreaded app might set the GPU’s execution order for a sequence of related subtasks by:

1. Creating a command buffer for each subtask

2. Enqueuing the command buffers in the proper order on a single thread

3. Encoding commands to each buffer on a separate thread and then committing it

## Topics

### Creating command encoders
- [Command encoder factory methods](https://developer.apple.com/documentation/metal/command-encoder-factory-methods) — A command encoder defines the actions of a single pass, such as GPU commands that draw, compute, or quickly copy resource data.

### Attaching residency sets
- [useResidencySet(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/useresidencyset(_:)) — Applies a residency set to a command buffer.
- [useResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/useresidencysets(_:)) — Applies multiple residency sets to a command buffer.

### Synchronizing passes with events
- [encodeWaitForEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodewaitforevent(_:value:)) — Encodes a command into the command buffer that pauses the GPU from running the buffer’s subsequent passes until the event equals or exceeds a value.
- [encodeSignalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodesignalevent(_:value:)) — Encodes a command that updates an event’s value, which can clear the GPU to run passes from other command buffers waiting for the event.

### Presenting a drawable
- [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)) — Presents a drawable as early as possible.
- [present(_:atTime:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:attime:)) — Presents a drawable at a specific time.
- [present(_:afterMinimumDuration:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:afterminimumduration:)) — Presents a drawable after the system presents the previous drawable for an amount of time.

### Registering state change handlers
- [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) — Registers a completion handler the GPU device calls immediately after it schedules the command buffer to run on the GPU.
- [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) — Registers a completion handler the GPU device calls immediately after the GPU finishes running the commands in the command buffer.
- [MTLCommandBufferHandler](https://developer.apple.com/documentation/metal/mtlcommandbufferhandler) — A completion handler signature a GPU device calls when it finishes scheduling a command buffer, or when the GPU finishes running it.

### Submitting a command buffer
- [enqueue()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/enqueue()) — Reserves the next available place for the command buffer in its command queue.
- [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) — Submits the command buffer to run on the GPU.

### Waiting for state changes
- [waitUntilScheduled()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilscheduled()) — Blocks the current thread until the command queue schedules the buffer.
- [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) — Blocks the current thread until the GPU finishes executing the command buffer and all of its completion handlers.

### Troubleshooting a command buffer
- [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) — The command buffer’s current state.
- [MTLCommandBufferStatus](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus) — The discrete states for a command buffer that represent its life cycle stages.
- [Command buffer debugging](https://developer.apple.com/documentation/metal/command-buffer-debugging) — Properties and methods for programmatically debugging runtime issues with a command buffer.

### Instance Methods
- [completed()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/completed())
- [scheduled()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/scheduled())

## See also

### Submitting work to a GPU with Metal
- [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure) — Discover how Metal executes commands on a GPU.
- [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) — An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
- [MTLCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor) — A configuration that customizes the behavior for a new command queue.
- [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) — A configuration that customizes the behavior for a new command buffer.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) — An encoder that writes GPU commands into a command buffer.
