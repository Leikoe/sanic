# MTL4RenderEncoderOptions

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderencoderoptions>

Custom render pass options you specify at encoder creation time.

## Declaration

```swift
struct MTL4RenderEncoderOptions
```

## Overview

Use these options to implement parallel encoding of render passes across multiple CPU threads by providing these values to the `options` parameter of [makeRenderCommandEncoder(descriptor:options:)](https://developer.apple.com/documentation/metal/mtl4commandbuffer/makerendercommandencoder(descriptor:options:)) and observing these requirements:

1. Commit all command encoders together in an array you provide to [commit:count:](https://developer.apple.com/documentation/metal/mtl4commandqueue/commit:count:) or [commit:count:options:](https://developer.apple.com/documentation/metal/mtl4commandqueue/commit:count:options:)

2. The first command buffer in the array contains a render pass that you start with option [suspending](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/suspending)

3. The last command buffer in the array contains the same render pass that you start with option [resuming](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/resuming)

4. All intermediate command buffers between the first and last in the array contain the same render pass that you start with both [resuming](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/resuming) and [suspending](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/suspending) options.

5. The sequence of render passes, in submission order, doesn’t intermix with compute, blit, acceleration structure or machine learning encoding.

6. A command buffer shouldn’t contain a render pass that you start with option [suspending](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/suspending) if it already contains a render pass that you start with option [resuming](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/resuming).

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/init(rawvalue:))

### Type Properties
- [resuming](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/resuming) — Configures the render pass to as *resuming*.
- [suspending](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions/suspending) — Configures the render pass as *suspending*.

## See also

### Submitting work to a GPU with Metal 4
- [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) — An abstraction representing a command queue that you use commit and synchronize command buffers and to perform other GPU operations.
- [MTL4CommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor) — Groups together parameters for the creation of a new command queue.
- [MTL4CommandQueueError](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct)
- [MTL4CommandQueueError.Code](https://developer.apple.com/documentation/metal/mtl4commandqueueerror-swift.struct/code) — Enumeration of kinds of errors that committing an array of command buffers instances can produce.
- [MTL4CommandQueueErrorDomain](https://developer.apple.com/documentation/metal/mtl4commandqueueerrordomain)
- [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) — Records a sequence of GPU commands.
- [MTL4CommandBufferOptions](https://developer.apple.com/documentation/metal/mtl4commandbufferoptions) — Options to configure a command buffer before encoding work into it.
- [MTL4CommandEncoder](https://developer.apple.com/documentation/metal/mtl4commandencoder) — An encoder that writes GPU commands into a command buffer.
- [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) — Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.
- [MTL4ArgumentTableDescriptor](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor) — Groups parameters for the creation of a Metal argument table.
- [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) — Manages the memory backing the encoding of GPU commands into command buffers.
- [MTL4CommandAllocatorDescriptor](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor) — Groups together parameters for creating a command allocator.
- [MTL4CommitOptions](https://developer.apple.com/documentation/metal/mtl4commitoptions) — Represents options to configure a commit operation on a command queue.
- [MTL4CommitFeedback](https://developer.apple.com/documentation/metal/mtl4commitfeedback) — Describes an object containing debug information from Metal to your app after completing a workload.
- [MTL4CommitFeedbackHandler](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler) — Defines the block signature for a callback Metal invokes to provide your app feedback after completing a workload.
