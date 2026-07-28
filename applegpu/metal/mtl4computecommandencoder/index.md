# MTL4ComputeCommandEncoder

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder>

Encodes computation dispatches, resource copying commands, and acceleration structure building commands for a single pass into a command buffer.

## Declaration

```swift
protocol MTL4ComputeCommandEncoder : MTL4CommandEncoder
```

## Overview

Each Metal 4 compute encoder combines compute dispatch commands, blit commands, and acceleration structure commands into a single pass. The unified nature of this encoder type eliminates the overhead from creating separate encoders like [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder), [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder), and [MTLAccelerationStructureCommandEncoder](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder), and then encoding separate passes with them.

Create a compute encoder by calling a factory method of an [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) instance, such as [makeComputeCommandEncoder()](https://developer.apple.com/documentation/metal/mtl4commandbuffer/makecomputecommandencoder()).

### Command stages

Most compute commands apply to one stage within a pass. The following table shows which stage applies to each command:

| Function | MTLStages |
|---|---|
| [dispatchThreads(threadsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(threadspergrid:threadsperthreadgroup:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [dispatchThreads(indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(indirectbuffer:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [dispatchThreadgroups(threadgroupsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(threadgroupspergrid:threadsperthreadgroup:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [dispatchThreadgroups(indirectBuffer:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(indirectbuffer:threadsperthreadgroup:)) | [dispatch](https://developer.apple.com/documentation/metal/mtlstages/dispatch) |
| [copy(sourceBuffer:sourceOffset:destinationBuffer:destinationOffset:size:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcebuffer:sourceoffset:destinationbuffer:destinationoffset:size:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(sourceBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcebuffer:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:options:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(sourceTensor:sourceOrigin:sourceDimensions:destinationTensor:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetensor:sourceorigin:sourcedimensions:destinationtensor:destinationorigin:destinationdimensions:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(sourceTexture:destinationTexture:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:destinationtexture:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(sourceTexture:sourceSlice:sourceLevel:destinationTexture:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:destinationtexture:destinationslice:destinationlevel:slicecount:levelcount:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationbuffer:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copyCommands(sourceBuffer:sourceRange:destinationBuffer:destinationIndex:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copycommands(sourcebuffer:sourcerange:destinationbuffer:destinationindex:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [fill(buffer:range:value:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/fill(buffer:range:value:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [generateMipmaps(texture:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/generatemipmaps(texture:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecommands(buffer:range:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContents(forCPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContents(forCPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:slice:level:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContents(forGPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContents(forGPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:slice:level:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [resetCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/resetcommands(buffer:range:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [build(destinationAccelerationStructure:descriptor:scratchBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/build(destinationaccelerationstructure:descriptor:scratchbuffer:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [refit(sourceAccelerationStructure:descriptor:destinationAccelerationStructure:scratchBuffer:options:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/refit(sourceaccelerationstructure:descriptor:destinationaccelerationstructure:scratchbuffer:options:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [writeCompactedSize(sourceAccelerationStructure:destinationBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writecompactedsize(sourceaccelerationstructure:destinationbuffer:)) | [accelerationStructure](https://developer.apple.com/documentation/metal/mtlstages/accelerationstructure) |
| [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:range:))![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[executeCommandsInBuffer:withRange:](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommandsinbuffer:withrange:) | None |
| [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:indirectbuffer:)) | None |
| [writeTimestamp(granularity:counterHeap:index:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writetimestamp(granularity:counterheap:index:)) | None |

The [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:range:)) and [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:indirectbuffer:)) commands don’t apply to any stage, which means you can’t use a barrier to wait for all commands in an indirect command buffer to complete. However, each command within the [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) applies to the same stages as when you encode the equivalent command directly.

For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Configuring the pass
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setcomputepipelinestate(_:)) — Configures this encoder with a compute pipeline state that applies to your subsequent dispatch commands.
- [setArgumentTable(_:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setargumenttable(_:)) — Sets an argument table for the compute shader stage of this pipeline.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setthreadgroupmemorylength(_:index:)) — Configures the size of a threadgroup memory buffer for a threadgroup argument in the compute shader function.
- [setImageblockSize(width:height:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setimageblocksize(width:height:)) — Specifies the size, in pixels, of imageblock data in tile memory.

### Inspecting the pass
- [stages()](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/stages()) — Queries a bitmask representing the shader stages on which commands currently present in this command encoder operate.

### Running dispatch commands
- [dispatchThreads(threadsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(threadspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command using an arbitrarily-sized grid.
- [dispatchThreads(indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(indirectbuffer:)) — Encodes a compute dispatch command with an arbitrarily sized grid, using an indirect buffer for arguments.
- [dispatchThreadgroups(threadgroupsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(threadgroupspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries.
- [dispatchThreadgroups(indirectBuffer:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(indirectbuffer:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries, using an indirect buffer for arguments.

### Encoding buffer copy commands
- [copy(sourceBuffer:sourceOffset:destinationBuffer:destinationOffset:size:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcebuffer:sourceoffset:destinationbuffer:destinationoffset:size:)) — Encodes a command that copies data from a buffer instance into another.

### Encoding buffer-to-texture copy commands
- [copy(sourceBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcebuffer:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:options:)) — Encodes a command to copy image data from a buffer into a texture with options for special texture formats.

### Encoding texture copy commands
- [copy(sourceTensor:sourceOrigin:sourceDimensions:destinationTensor:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetensor:sourceorigin:sourcedimensions:destinationtensor:destinationorigin:destinationdimensions:)) — Encodes a command to copy data from a slice of the data plane of a tensor into a slice of the data plane of another tensor.
- [copy(sourceTexture:destinationTexture:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:destinationtexture:)) — Encodes a command that copies data from a texture to another.
- [copy(sourceTexture:sourceSlice:sourceLevel:destinationTexture:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:destinationtexture:destinationslice:destinationlevel:slicecount:levelcount:)) — Encodes a command that copies slices of a texture to slices of another texture.
- [copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:)) — Encodes a command that copies image data from a slice of a texture into a slice of another texture.

### Encoding texture-to-buffer copy commands
- [copy(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationbuffer:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) — Encodes a command that copies image data from a slice of a texture instance to a buffer, with options for special texture formats.

### Encoding indirect command buffer copy commands
- [copyCommands(sourceBuffer:sourceRange:destinationBuffer:destinationIndex:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copycommands(sourcebuffer:sourcerange:destinationbuffer:destinationindex:)) — Encodes a command that copies commands from one indirect command buffer into another.

### Encoding buffer fill commands
- [fill(buffer:range:value:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/fill(buffer:range:value:)) — Encodes a command that fills a buffer with a constant value for each byte.

### Encoding mipmap generation commands
- [generateMipmaps(texture:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/generatemipmaps(texture:)) — Encodes a command that generates mipmaps for a texture instance from the base mipmap level up to the highest mipmap level.

### Encoding optimization commands
- [optimizeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecommands(buffer:range:)) — Encode a command to attempt to improve the performance of a range of commands within an indirect command buffer.
- [optimizeContents(forCPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:)) — Encodes a command that modifies the contents of a texture to improve the performance of CPU accesses to its contents.
- [optimizeContents(forCPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forcpuaccess:slice:level:)) — Encodes a command that modifies the contents of a texture to improve the performance of CPU accesses to its contents in a specific region.
- [optimizeContents(forGPUAccess:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:)) — Encodes a command that modifies the contents of a texture to improve the performance of GPU accesses to its contents.
- [optimizeContents(forGPUAccess:slice:level:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/optimizecontents(forgpuaccess:slice:level:)) — Encodes a command that modifies the contents of a texture instance to improve the performance of GPU accesses to its contents in a specific region.

### Encoding reset commands
- [resetCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/resetcommands(buffer:range:)) — Encodes a command that resets a range of commands in an indirect command buffer.

### Encoding acceleration structure build commands
- [build(destinationAccelerationStructure:descriptor:scratchBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/build(destinationaccelerationstructure:descriptor:scratchbuffer:)) — Encodes an acceleration structure build into the command buffer.

### Encoding acceleration structure copy commands
- [copy(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes an acceleration structure copy operation into the command buffer.
- [copyAndCompact(sourceAccelerationStructure:destinationAccelerationStructure:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copyandcompact(sourceaccelerationstructure:destinationaccelerationstructure:)) — Encodes a command to copy and compact an acceleration structure.
- [writeCompactedSize(sourceAccelerationStructure:destinationBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writecompactedsize(sourceaccelerationstructure:destinationbuffer:)) — Encodes a command to compute the size an acceleration structure can compact into, writing the result into a buffer.

### Encoding acceleration structure refit commands
- [refit(sourceAccelerationStructure:descriptor:destinationAccelerationStructure:scratchBuffer:options:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/refit(sourceaccelerationstructure:descriptor:destinationaccelerationstructure:scratchbuffer:options:)) — Encodes an acceleration structure refit operation into the command buffer, providing additional options.

### Encoding indirect command buffers
- [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:range:)) — Encodes a command to execute commands from an indirect command buffer.
- [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:indirectbuffer:)) — Encodes an instruction to execute commands from an indirect command buffer, using an indirect buffer for arguments.

### Encoding performance measurement commands
- [writeTimestamp(granularity:counterHeap:index:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/writetimestamp(granularity:counterheap:index:)) — Writes a GPU timestamp into a heap.

### Instance Methods
- [copy(sourceTensor:sourceOrigin:sourceDimensions:sourcePlane:destinationTensor:destinationOrigin:destinationDimensions:destinationPlane:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/copy(sourcetensor:sourceorigin:sourcedimensions:sourceplane:destinationtensor:destinationorigin:destinationdimensions:destinationplane:)) — Encodes a command to copy data from a slice of a plane of a tensor into a slice of a plane of another tensor.

## See also

### Encoding a compute pass
- [Creating threads and threadgroups](https://developer.apple.com/documentation/metal/creating-threads-and-threadgroups) — Learn how Metal organizes compute-processing workloads.
- [Calculating threadgroup and grid sizes](https://developer.apple.com/documentation/metal/calculating-threadgroup-and-grid-sizes) — Calculate the optimum sizes for threadgroups and grids when dispatching compute-processing workloads.
- [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) — Encodes computation dispatch commands for a single compute pass into a command buffer.
