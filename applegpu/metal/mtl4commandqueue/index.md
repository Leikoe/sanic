# MTL4CommandQueue

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue>

An abstraction representing a command queue that you use commit and synchronize command buffers and to perform other GPU operations.

## Declaration

```swift
protocol MTL4CommandQueue : NSObjectProtocol, Sendable
```

## Topics

### Instance Properties
- [device](https://developer.apple.com/documentation/metal/mtl4commandqueue/device) — Returns the GPU device that the command queue belongs to.
- [label](https://developer.apple.com/documentation/metal/mtl4commandqueue/label) — Obtains this queue’s optional label for debugging purposes.

### Instance Methods
- [addResidencySet(_:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/addresidencyset(_:)) — Applies a residency set to a queue, which Metal applies to the queue’s command buffers as you commit them.
- [addResidencySets(_:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/addresidencysets(_:)) — Applies multiple residency sets to a queue, which Metal applies to the queue’s command buffers as you commit them.
- [commit(_:options:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/commit(_:options:)) — Enqueues an array of command buffer instances for execution with a set of options.
- [copyMappings(sourceBuffer:destinationBuffer:operations:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/copymappings(sourcebuffer:destinationbuffer:operations:)) — Copies multiple offsets within a source placement sparse buffer to a destination placement sparse buffer.
- [copyMappings(sourceTexture:destinationTexture:operations:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/copymappings(sourcetexture:destinationtexture:operations:)) — Copies multiple regions within a source placement sparse texture to a destination placement sparse texture.
- [removeResidencySet(_:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/removeresidencyset(_:)) — Removes a residency set from a command queue’s list, which means Metal doesn’t apply it to the queue’s command buffers as you commit them.
- [removeResidencySets(_:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/removeresidencysets(_:)) — Removes multiple residency sets from a command queue’s list, which means Metal doesn’t apply them to the queue’s command buffers as you commit them.
- [signalDrawable(_:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/signaldrawable(_:)) — Schedules a signal operation on the command queue to indicate when rendering to a Metal drawable is complete.
- [signalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/signalevent(_:value:)) — Schedules an operation to signal a GPU event with a specific value after all GPU work prior to this point is complete.
- [updateMappings(buffer:heap:operations:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/updatemappings(buffer:heap:operations:)) — Updates multiple regions within a placement sparse buffer to alias specific tiles from a Metal heap.
- [updateMappings(texture:heap:operations:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/updatemappings(texture:heap:operations:)) — Updates multiple regions within a placement sparse texture to alias specific tiles of a Metal heap.
- [waitForDrawable(_:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/waitfordrawable(_:)) — Schedules a wait operation on the command queue to ensure the display is no longer using a specific Metal drawable.
- [waitForEvent(_:value:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/waitforevent(_:value:)) — Schedules an operation to wait for a GPU event of a specific value before continuing to execute any future GPU work.

## See also

### Submitting work to a GPU with Metal 4
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
