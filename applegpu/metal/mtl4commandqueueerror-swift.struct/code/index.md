# MTL4CommandQueueError.Code

*Enumeration · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code>

Enumeration of kinds of errors that committing an array of command buffers instances can produce.

## Declaration

```swift
enum Code
```

## Topics

### Enumeration Cases
- [MTL4CommandQueueError.Code.accessRevoked](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/accessrevoked) — Indicates that the system revokes GPU access because it’s responsible for too many timeouts or hangs.
- [MTL4CommandQueueError.Code.deviceRemoved](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/deviceremoved) — Indicates the physical removal of the GPU before the command buffer completed.
- [MTL4CommandQueueError.Code.internal](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/internal) — Indicates an internal problem in the Metal framework.
- [MTL4CommandQueueError.Code.none](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/none) — Indicates the absence of any problems.
- [MTL4CommandQueueError.Code.notPermitted](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/notpermitted) — Indicates a process doesn’t have access to a GPU device.
- [MTL4CommandQueueError.Code.outOfMemory](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/outofmemory) — Indicates the GPU doesn’t have sufficient memory to execute a command buffer.
- [MTL4CommandQueueError.Code.timeout](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/timeout) — Indicates the workload takes longer to execute than the system allows.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code/init(rawvalue:))

## See also

### Submitting work to a GPU with Metal 4
- [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) — An abstraction representing a command queue that you use commit and synchronize command buffers and to perform other GPU operations.
- [MTL4CommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor) — Groups together parameters for the creation of a new command queue.
- [MTL4CommandQueueError](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct)
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
