# MTL4ArgumentTable

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4argumenttable>

Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.

## Declaration

```swift
protocol MTL4ArgumentTable : NSObjectProtocol
```

## Topics

### Instance Properties
- [device](https://developer.apple.com/documentation/metal/mtl4argumenttable/device) — The device from which you created this argument table.
- [label](https://developer.apple.com/documentation/metal/mtl4argumenttable/label) — Assigns an optional label with this argument table for debugging purposes.

### Instance Methods
- [setAddress(_:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtl4argumenttable/setaddress(_:attributestride:index:)) — Binds a GPU address to a buffer binding slot, providing a dynamic vertex stride.
- [setAddress(_:index:)](https://developer.apple.com/documentation/metal/mtl4argumenttable/setaddress(_:index:)) — Binds a GPU address to a buffer binding slot.
- [setResource(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtl4argumenttable/setresource(_:bufferindex:)) — Binds a resource to a buffer binding slot.
- [setSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtl4argumenttable/setsamplerstate(_:index:)) — Binds a sampler state to a sampler state binding slot.
- [setTexture(_:index:)](https://developer.apple.com/documentation/metal/mtl4argumenttable/settexture(_:index:)) — Binds a texture to a texture binding slot.

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
- [MTL4ArgumentTableDescriptor](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor) — Groups parameters for the creation of a Metal argument table.
- [MTL4CommandAllocator](https://developer.apple.com/documentation/metal/mtl4commandallocator) — Manages the memory backing the encoding of GPU commands into command buffers.
- [MTL4CommandAllocatorDescriptor](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor) — Groups together parameters for creating a command allocator.
- [MTL4CommitOptions](https://developer.apple.com/documentation/metal/mtl4commitoptions) — Represents options to configure a commit operation on a command queue.
- [MTL4CommitFeedback](https://developer.apple.com/documentation/metal/mtl4commitfeedback) — Describes an object containing debug information from Metal to your app after completing a workload.
- [MTL4CommitFeedbackHandler](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler) — Defines the block signature for a callback Metal invokes to provide your app feedback after completing a workload.
