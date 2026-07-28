# MTLBlitCommandEncoder

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlblitcommandencoder>

Encodes commands that copy and modify resources for a single blit pass.

## Declaration

```swift
protocol MTLBlitCommandEncoder : MTLCommandEncoder
```

## Overview

Create a blit encoder by calling one of the factory methods on an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance, such as [makeBlitCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeblitcommandencoder()).

A blit command encoder adds commands to a command buffer that modify resources in various ways, including:

- Filling buffers with repeating bytes

- Generating mipmaps for textures

- Copying data between buffers

- Copying data between textures

- Copying data between a texture and a buffer

- Managing the contents of indirect command buffers

- Synchronizing buffers, textures, and other resources between the CPU and GPU

- Improving runtime performance for resources by optimizing their memory layout for the GPU or CPU

You typically use these commands to move data between a resource that uses private storage and another resource that uses CPU-accessible storage. Some apps also use them to apply image-processing and texture effects, such as blurring or reflections, or to render and work with offscreen image data.

When you finish encoding blit commands, finalize the blit pass into the command buffer by calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method.

### Command stages

Most blit commands apply to one stage within a pass. The following table shows which stages apply to each command:

| Function | MTLStages |
|---|---|
| [fill(buffer:range:value:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/fill(buffer:range:value:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [generateMipmaps(for:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/generatemipmaps(for:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceOffset:to:destinationOffset:size:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:to:destinationoffset:size:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:to:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:to:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceSlice:sourceLevel:to:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:to:destinationslice:destinationlevel:slicecount:levelcount:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationslice:destinationlevel:destinationorigin:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceOrigin:sourceDimensions:to:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceorigin:sourcedimensions:to:destinationorigin:destinationdimensions:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContentsForGPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContentsForGPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:slice:level:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContentsForCPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeContentsForCPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:slice:level:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) | None |
| [synchronize(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)) | None |
| [copyIndirectCommandBuffer(_:sourceRange:destination:destinationIndex:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copyindirectcommandbuffer(_:sourcerange:destination:destinationindex:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [resetCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resetcommandsinbuffer(_:range:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [optimizeIndirectCommandBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizeindirectcommandbuffer(_:range:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) | None |
| [resolveCounters(_:range:destinationBuffer:destinationOffset:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resolvecounters(_:range:destinationbuffer:destinationoffset:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [getTextureAccessCounters(_:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/gettextureaccesscounters(_:region:miplevel:slice:resetcounters:countersbuffer:countersbufferoffset:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |
| [resetTextureAccessCounters(_:region:mipLevel:slice:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resettextureaccesscounters(_:region:miplevel:slice:)) | [blit](https://developer.apple.com/documentation/metal/mtlstages/blit) |

For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Filling buffers
- [fill(buffer:range:value:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/fill(buffer:range:value:)) — Encodes a command that fills a buffer with a constant value for each byte.

### Generating texture mipmaps
- [generateMipmaps(for:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/generatemipmaps(for:)) — Encodes a command that generates mipmaps for a texture from the base mipmap level up to the highest mipmap level.

### Copying buffer data to another buffer
- [copy(from:sourceOffset:to:destinationOffset:size:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:to:destinationoffset:size:)) — Encodes a command that copies data from one buffer into another.

### Copying texture data to another texture
- [copy(from:to:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:to:)) — Encodes a command that copies data from one texture to another.
- [copy(from:sourceSlice:sourceLevel:to:destinationSlice:destinationLevel:sliceCount:levelCount:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:to:destinationslice:destinationlevel:slicecount:levelcount:)) — Encodes a command that copies slices of a texture to another texture’s slices.
- [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationslice:destinationlevel:destinationorigin:)) — Encodes a command that copies image data from a texture’s slice into another slice.
- [copy(from:sourceOrigin:sourceDimensions:to:destinationOrigin:destinationDimensions:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceorigin:sourcedimensions:to:destinationorigin:destinationdimensions:)) — Encodes a command to copy data from a slice of the data plane of a tensor into a slice of the data plane of another tensor.

### Copying buffer data to a texture
- [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:)) — Encodes a command to copy image data from a source buffer into a destination texture.
- [copy(from:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:to:destinationSlice:destinationLevel:destinationOrigin:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceoffset:sourcebytesperrow:sourcebytesperimage:sourcesize:to:destinationslice:destinationlevel:destinationorigin:options:)) — Encodes a command to copy image data from a source buffer into a destination texture.

### Copying texture data to a buffer
- [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:)) — Encodes a command that copies image data from a texture slice to a buffer.
- [copy(from:sourceSlice:sourceLevel:sourceOrigin:sourceSize:to:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceslice:sourcelevel:sourceorigin:sourcesize:to:destinationoffset:destinationbytesperrow:destinationbytesperimage:options:)) — Encodes a command that copies image data from a texture slice to a buffer, and provides options for special texture formats.

### Optimizing textures for GPU access
- [optimizeContentsForGPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:)) — Encodes a command that improves the performance of GPU memory operations with a texture.
- [optimizeContentsForGPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforgpuaccess(texture:slice:level:)) — Encodes a command that improves the performance of GPU memory operations with a specific portion of a texture.

### Optimizing textures for CPU access
- [optimizeContentsForCPUAccess(texture:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:)) — Encodes a command that improves the performance of CPU memory operations with a texture.
- [optimizeContentsForCPUAccess(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizecontentsforcpuaccess(texture:slice:level:)) — Encodes a command that improves the performance of CPU memory operations with a specific portion of a texture.

### Synchronizing managed resources
- [synchronize(resource:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(resource:)) — Encodes a command that synchronizes the CPU’s copy of a managed resource, such as a buffer or texture, so that it matches the GPU’s copy.
- [synchronize(texture:slice:level:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/synchronize(texture:slice:level:)) — Encodes a command that synchronizes a part of the CPU’s copy of a texture so that it matches the GPU’s copy.

### Preventing resource access conflicts
- [waitForFence(_:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/waitforfence(_:)) — Encodes a command that instructs the GPU to pause the blit pass until another pass updates a fence.
- [updateFence(_:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/updatefence(_:)) — Encodes a command that instructs the GPU to update a fence after the blit pass completes.

### Managing indirect command buffers
- [copyIndirectCommandBuffer(_:sourceRange:destination:destinationIndex:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copyindirectcommandbuffer(_:sourcerange:destination:destinationindex:)) — Encodes a command that copies commands from one indirect command buffer into another.
- [resetCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resetcommandsinbuffer(_:range:)) — Encodes a command that resets a range of commands in an indirect command buffer.
- [optimizeIndirectCommandBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/optimizeindirectcommandbuffer(_:range:)) — Encodes a command that can improve the performance of a range of commands within an indirect command buffer.

### Sampling counters
- [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) — Encodes a command that samples the GPU’s hardware counters during a blit pass and stores the data in a counter sample buffer.
- [resolveCounters(_:range:destinationBuffer:destinationOffset:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resolvecounters(_:range:destinationbuffer:destinationoffset:)) — Encodes a command that resolves the data from the samples in a sample counter buffer and stores the results into a buffer.

### Managing sparse texture access counters
- [getTextureAccessCounters(_:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/gettextureaccesscounters(_:region:miplevel:slice:resetcounters:countersbuffer:countersbufferoffset:)) — Encodes a command that retrieves a sparse texture’s access data for a specific region, mipmap level, and slice.
- [resetTextureAccessCounters(_:region:mipLevel:slice:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/resettextureaccesscounters(_:region:miplevel:slice:)) — Encodes a command that resets a sparse texture’s access data for a specific region, mipmap level, and slice.

### Instance Methods
- [copy(from:sourceOrigin:sourceDimensions:sourcePlane:to:destinationOrigin:destinationDimensions:destinationPlane:)](https://developer.apple.com/documentation/metal/mtlblitcommandencoder/copy(from:sourceorigin:sourcedimensions:sourceplane:to:destinationorigin:destinationdimensions:destinationplane:)) — Encodes a command to copy data from a slice of a plane of a tensor into a slice of a plane of another tensor.

## See also

### Encoding a blit pass
- [MTLBlitOption](https://developer.apple.com/documentation/metal/mtlblitoption) — The options that enable behavior for some blit operations.
