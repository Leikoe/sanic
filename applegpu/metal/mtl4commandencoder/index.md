# MTL4CommandEncoder

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandencoder>

An encoder that writes GPU commands into a command buffer.

## Declaration

```swift
protocol MTL4CommandEncoder : NSObjectProtocol
```

## Topics

### Instance Properties
- [commandBuffer](https://developer.apple.com/documentation/metal/mtl4commandencoder/commandbuffer) — Returns the command buffer that is currently encoding commands.
- [label](https://developer.apple.com/documentation/metal/mtl4commandencoder/label) — Provides an optional label to assign to the command encoder for debug purposes.

### Instance Methods
- [barrier(afterEncoderStages:beforeEncoderStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterencoderstages:beforeencoderstages:visibilityoptions:)) — Encodes an intra-pass barrier.
- [barrier(afterQueueStages:beforeStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterqueuestages:beforestages:visibilityoptions:)) — Encodes a consumer barrier on work you commit to the same command queue.
- [barrier(afterStages:beforeQueueStages:visibilityOptions:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/barrier(afterstages:beforequeuestages:visibilityoptions:)) — Encodes a producer barrier on work committed to the same command queue.
- [endEncoding()](https://developer.apple.com/documentation/metal/mtl4commandencoder/endencoding()) — Declares that all command generation from this encoder is complete.
- [insertDebugSignpost(_:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/insertdebugsignpost(_:)) — Inserts a debug string into the frame data to aid debugging.
- [popDebugGroup()](https://developer.apple.com/documentation/metal/mtl4commandencoder/popdebuggroup()) — Pops the latest debug group string from this encoder’s stack of debug groups.
- [pushDebugGroup(_:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/pushdebuggroup(_:)) — Pushes a string onto this encoder’s stack of debug groups.
- [updateFence(_:afterEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/updatefence(_:afterencoderstages:)) — Encodes a command that instructs the GPU to update a fence after one or more stages, which can unblock other passes waiting for the fence.
- [waitForFence(_:beforeEncoderStages:)](https://developer.apple.com/documentation/metal/mtl4commandencoder/waitforfence(_:beforeencoderstages:)) — Encodes a command that instructs the GPU to pause before starting one or more stages of the pass until a pass updates a fence.

## See also

### Submitting work to a GPU with Metal 4
- [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) — An abstraction representing a command queue that you use commit and synchronize command buffers and to perform other GPU operations.
- [MTL4CommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor) — Groups together parameters for the creation of a new command queue.
- [MTL4CommandQueueError](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct)
- [MTL4CommandQueueError.Code](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code) — Enumeration of kinds of errors that committing an array of command buffers instances can produce.
- [MTL4CommandQueueErrorDomain](https://developer.apple.com/documentation/metal/mtl4commandqueueerrordomain)
- [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) — Records a sequence of GPU commands.
- [MTL4CommandBufferOptions](https://developer.apple.com/documentation/metal/mtl4commandbufferoptions) — Options to configure a command buffer before encoding work into it.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) — Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.
- [MTL4ArgumentTableDescriptor](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor) — Groups parameters for the creation of a Metal argument table.
- [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) — Manages the memory backing the encoding of GPU commands into command buffers.
- [MTL4CommandAllocatorDescriptor](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor) — Groups together parameters for creating a command allocator.
- [MTL4CommitOptions](https://developer.apple.com/documentation/metal/mtl4commitoptions) — Represents options to configure a commit operation on a command queue.
- [MTL4CommitFeedback](https://developer.apple.com/documentation/metal/mtl4commitfeedback) — Describes an object containing debug information from Metal to your app after completing a workload.
- [MTL4CommitFeedbackHandler](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler) — Defines the block signature for a callback Metal invokes to provide your app feedback after completing a workload.
