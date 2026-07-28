# GPU devices and work submission

*API Collection*

<https://developer.apple.com/documentation/metal/gpu-devices-and-work-submission>

Find any available GPU, submit work to it with command buffers, suspend work, and coordinate between multiple GPUs.

## Overview

You can use any available GPU’s [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance in addition to the default instance that [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()) returns. For each device instance, get its [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance, and create one or more [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instances to send work to the GPU.

When the system suspends your app, use the command queue to finish command buffers already in progress. See [Preparing your Metal app to run in the background](https://developer.apple.com/documentation/metal/preparing-your-metal-app-to-run-in-the-background) for more information.

## Topics

### Locating and inspecting a GPU device
- [Getting the default GPU](https://developer.apple.com/documentation/metal/getting-the-default-gpu) — Select the system’s default GPU device on which to run your Metal code.
- [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) — Use the device object’s properties to determine how you perform tasks in Metal.
- [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()) — Returns the device instance Metal selects as the default.
- [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) — The main Metal interface to a GPU that apps use to draw graphics and run computations in parallel.
- [Multi-GPU systems](https://developer.apple.com/documentation/metal/multi-gpu-systems) — Locate and work with internal and external GPUs and their displays, video memory, and performance tradeoffs.

### Submitting work to a GPU with Metal 4
- [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) — An abstraction representing a command queue that you use commit and synchronize command buffers and to perform other GPU operations.
- [MTL4CommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor) — Groups together parameters for the creation of a new command queue.
- [MTL4CommandQueueError](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct)
- [MTL4CommandQueueError.Code](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code) — Enumeration of kinds of errors that committing an array of command buffers instances can produce.
- [MTL4CommandQueueErrorDomain](https://developer.apple.com/documentation/metal/mtl4commandqueueerrordomain)
- [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) — Records a sequence of GPU commands.
- [MTL4CommandBufferOptions](https://developer.apple.com/documentation/metal/mtl4commandbufferoptions) — Options to configure a command buffer before encoding work into it.
- [MTL4CommandEncoder](https://developer.apple.com/documentation/metal/mtl4commandencoder) — An encoder that writes GPU commands into a command buffer.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) — Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.
- [MTL4ArgumentTableDescriptor](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor) — Groups parameters for the creation of a Metal argument table.
- [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) — Manages the memory backing the encoding of GPU commands into command buffers.
- [MTL4CommandAllocatorDescriptor](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor) — Groups together parameters for creating a command allocator.
- [MTL4CommitOptions](https://developer.apple.com/documentation/metal/mtl4commitoptions) — Represents options to configure a commit operation on a command queue.
- [MTL4CommitFeedback](https://developer.apple.com/documentation/metal/mtl4commitfeedback) — Describes an object containing debug information from Metal to your app after completing a workload.
- [MTL4CommitFeedbackHandler](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler) — Defines the block signature for a callback Metal invokes to provide your app feedback after completing a workload.
- [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) — Represents an opaque, driver-controlled section of memory that can store GPU counter data.
- [MTL4CounterHeapDescriptor](https://developer.apple.com/documentation/metal/mtl4counterheapdescriptor) — Groups together parameters for configuring a counter heap object at creation time.
- [MTL4CounterHeapType](https://developer.apple.com/documentation/metal/mtl4counterheaptype) — Defines the type of a [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) and the contents of its entries.
- [MTL4TimestampHeapEntry](https://developer.apple.com/documentation/metal/mtl4timestampheapentry) — Represents a timestamp data entry in a counter heap of type `MTL4CounterHeapTypeTimestamp`.
- [MTL4TimestampGranularity](https://developer.apple.com/documentation/metal/mtl4timestampgranularity) — Provides a hint to the system about the desired accuracy when writing GPU counter timestamps.

### Submitting work to a GPU with Metal
- [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure) — Discover how Metal executes commands on a GPU.
- [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) — An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
- [MTLCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor) — A configuration that customizes the behavior for a new command queue.
- [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) — A container that stores a sequence of GPU commands that you encode into it.
- [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) — A configuration that customizes the behavior for a new command buffer.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) — An encoder that writes GPU commands into a command buffer.

### Suspending work on a GPU
- [Preparing your Metal app to run in the background](https://developer.apple.com/documentation/metal/preparing-your-metal-app-to-run-in-the-background) — Prepare your app to move into the background by pausing future GPU use and ensuring previous work is scheduled.
