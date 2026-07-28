# MTLComputeCommandEncoder

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder>

Encodes computation dispatch commands for a single compute pass into a command buffer.

## Declaration

```swift
protocol MTLComputeCommandEncoder : MTLCommandEncoder
```

## Overview

Create a compute encoder by calling one of the factory methods on an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance, such as [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)). You can encode multiple commands that each run a compute kernel as part of a single pass of the encoder with the following steps:

1. Configure an [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) instance with a kernel, using a method such as [makeComputePipelineState(function:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:)). See the [Creating compute pipeline states](https://developer.apple.com/documentation/metal/pipeline-state-creation#Creating-compute-pipeline-states) section of [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation) for all [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods that create a new pipeline state for your command encoder.

2. Set the pipeline state with the [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)) method on your command encoder.

3. Set kernel arguments by binding buffers, textures, and other resources with methods such as [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:index:)) and [setTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settexture(_:index:)).

4. Encode compute commands that call your kernel by either [Dispatching kernel calls directly](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder#Dispatching-kernel-calls-directly) or [Dispatching from indirect command buffers](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder#Dispatching-from-indirect-command-buffers).

5. Call [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) to finish encoding the kernel call of the compute pass.

### Command stages

Most compute commands apply to one stage within a pass. The following table shows which stage applies to each command:

| Function | MTLStages |
|---|---|
| [dispatchThreads(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreads(_:threadsperthreadgroup:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [dispatchThreadgroups(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(_:threadsperthreadgroup:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:))![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[executeCommandsInBuffer:withRange:](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer:withrange:) | None |
| [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:))![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer:indirectbuffer:indirectbufferoffset:) | None |
| [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) | None |

The [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)) and [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) commands don’t apply to any stage, which means you can’t use a barrier to wait for all commands in an indirect command buffer to complete. However, each command within the [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) applies to the same stages as when you encode the equivalent command directly.

For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Configuring the pipeline state
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)) — Configures the compute encoder with a pipeline state for subsequent kernel calls.
- [dispatchType](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchtype) — The dispatch type to use when submitting compute work to the GPU.

### Binding buffers
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:index:)) — Binds a buffer to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:attributestride:index:)) — Binds a buffer with a stride to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:range:)) — Binds multiple buffers to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:attributestrides:range:)) — Binds multiple buffers with data in stride to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(_:index:)) — Changes where the data begins in a buffer already bound to the buffer argument table.
- [setBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(offset:attributestride:index:)) — Changes where the data begins and the distance between adjacent elements in a buffer already bound to the buffer argument table.

### Binding raw bytes
- [setBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbytes(_:length:index:)) — Copies data directly to the GPU to populate an entry in the buffer argument table.
- [setBytes(_:length:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbytes(_:length:attributestride:index:)) — Copies data with a given stride directly to the GPU to populate an entry in the buffer argument table.

### Binding textures
- [setTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settexture(_:index:)) — Binds a texture to the texture argument table, allowing compute kernels to access its data on the GPU.
- [setTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/settextures(_:range:)) — Binds multiple textures to the texture argument table, allowing compute functions to access their data on the GPU.

### Binding texture samplers
- [setSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstate(_:index:)) — Encodes a texture sampler, allowing compute kernels to use it for sampling textures on the GPU.
- [setSamplerState(_:lodMinClamp:lodMaxClamp:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstate(_:lodminclamp:lodmaxclamp:index:)) — Encodes a texture sampler with a custom level of detail clamping, allowing compute kernels to use it for sampling textures on the GPU.
- [setSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstates(_:range:)) — Encodes multiple texture samplers to the sampler argument table, allowing compute kernels to use them for sampling textures on the GPU.
- [setSamplerStates(_:lodMinClamps:lodMaxClamps:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setsamplerstates(_:lodminclamps:lodmaxclamps:range:)) — Encodes multiple texture samplers for the compute function, specifying clamp values for the level of detail of each sampler.

### Binding function tables
- [setVisibleFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setvisiblefunctiontable(_:bufferindex:)) — Binds a visible function table to the buffer argument table, allowing you to call its functions on the GPU.
- [setVisibleFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setvisiblefunctiontables(_:bufferrange:)) — Binds multiple visible function tables to the buffer argument table, allowing you to call their functions on the GPU.
- [setIntersectionFunctionTables(_:bufferRange:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setintersectionfunctiontables(_:bufferrange:)) — Binds multiple intersection function tables to the buffer argument table, allowing you to call their functions on the GPU.

### Binding arguments for acceleration structures
- [setAccelerationStructure(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setaccelerationstructure(_:bufferindex:)) — Binds an acceleration structure to the buffer argument table, allowing functions to access it on the GPU.
- [setIntersectionFunctionTable(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setintersectionfunctiontable(_:bufferindex:)) — Binds an intersection function table to the buffer argument table, making it callable in your Metal shaders.

### Making indirect resources resident
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) — Ensures kernel calls that the system encodes in subsequent commands have access to a resource.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresources(_:usage:)) — Ensures kernel calls that the system encodes in subsequent commands have access to multiple resources.
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to all of the resources you allocate from a heap.
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheaps(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to all of the resources you allocate from multiple heaps.

### Configuring tile memory
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setthreadgroupmemorylength(_:index:)) — Configures the size of a block of threadgroup memory.
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setimageblockwidth(_:height:)) — Sets the size, in pixels, of imageblock data in tile memory.

### Configuring stage-in data
- [setStageInRegion(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setstageinregion(_:)) — Sets the dimensions over the thread grid of how your compute kernel receives stage-in arguments.
- [setStageInRegionWithIndirectBuffer(_:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setstageinregionwithindirectbuffer(_:indirectbufferoffset:)) — Sets the region of the stage-in attributes to apply to a compute kernel using an indirect buffer.

### Dispatching kernel calls directly
- [dispatchThreads(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreads(_:threadsperthreadgroup:)) — Encodes a compute command using an arbitrarily sized grid.
- [dispatchThreadgroups(_:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(_:threadsperthreadgroup:)) — Encodes a compute dispatch command using a grid aligned to threadgroup boundaries.

### Dispatching from indirect command buffers
- [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) — Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)) — Encodes an instruction to run commands from an indirect buffer.
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:indirectbuffer:indirectbufferoffset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:with:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:with:)) — Encodes an instruction to run commands from an indirect buffer.

### Preventing resource access conflicts
- [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/waitforfence(_:)) — Encodes a command that instructs the GPU to pause the compute pass until another pass updates a fence.
- [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/updatefence(_:)) — Encodes a command that instructs the GPU to update a fence after the compute pass completes.
- [memoryBarrier(scope:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/memorybarrier(scope:)) — Creates a memory barrier that enforces the order of write and read operations for specific resource types.
- [memoryBarrier(resources:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/memorybarrier(resources:)) — Creates a memory barrier that enforces the order of write and read operations for specific resources.

### Sampling counters
- [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) — Encodes a command to sample hardware counters, providing performance information.

## See also

### Encoding a compute pass
- [Creating threads and threadgroups](https://developer.apple.com/documentation/metal/creating-threads-and-threadgroups) — Learn how Metal organizes compute-processing workloads.
- [Calculating threadgroup and grid sizes](https://developer.apple.com/documentation/metal/calculating-threadgroup-and-grid-sizes) — Calculate the optimum sizes for threadgroups and grids when dispatching compute-processing workloads.
- [MTL4ComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtl4computecommandencoder) — Encodes computation dispatches, resource copying commands, and acceleration structure building commands for a single pass into a command buffer.
