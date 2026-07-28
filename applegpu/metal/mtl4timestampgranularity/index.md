# MTL4TimestampGranularity

*Enumeration · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4timestampgranularity>

Provides a hint to the system about the desired accuracy when writing GPU counter timestamps.

## Declaration

```swift
enum MTL4TimestampGranularity
```

## Overview

Pass these values to [writeTimestamp(granularity:counterHeap:index:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writetimestamp(granularity:counterheap:index:)) and [writeTimestamp(granularity:after:counterHeap:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/writetimestamp(granularity:after:counterheap:index:)) to control the desired accurracy of the counter sampling operation.

## Topics

### Enumeration Cases
- [MTL4TimestampGranularity.precise](https://developer.apple.com/documentation/metal/mtl4timestampgranularity/precise) — A timestamp as precise as possible.
- [MTL4TimestampGranularity.relaxed](https://developer.apple.com/documentation/metal/mtl4timestampgranularity/relaxed) — A minimally-invasive timestamp which may be less precise.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtl4timestampgranularity/init(rawvalue:))

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
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) — Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.
- [MTL4ArgumentTableDescriptor](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor) — Groups parameters for the creation of a Metal argument table.
- [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) — Manages the memory backing the encoding of GPU commands into command buffers.
- [MTL4CommandAllocatorDescriptor](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor) — Groups together parameters for creating a command allocator.
- [MTL4CommitOptions](https://developer.apple.com/documentation/metal/mtl4commitoptions) — Represents options to configure a commit operation on a command queue.
- [MTL4CommitFeedback](https://developer.apple.com/documentation/metal/mtl4commitfeedback) — Describes an object containing debug information from Metal to your app after completing a workload.
