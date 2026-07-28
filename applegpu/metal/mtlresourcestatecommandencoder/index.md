# MTLResourceStateCommandEncoder

*Protocol · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder>

An encoder that encodes commands that modify resource configurations.

## Declaration

```swift
protocol MTLResourceStateCommandEncoder : MTLCommandEncoder
```

## Overview

Use a resource state command encoder to manage memory mappings for sparse textures.

Your app does not define classes that implement this protocol. To create an [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) instance, call the [makeResourceStateCommandEncoder()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makeresourcestatecommandencoder()) method of the [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance into which you want to encode blit commands. Next, call methods on the [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) instance to enqueue state updates. Finally, call [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) to finish the encoding process.

## Topics

### Updating texture memory assignments
- [updateTextureMapping(_:mode:region:mipLevel:slice:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemapping(_:mode:region:miplevel:slice:)) — Encodes a command to update the texture mappings for a region in a single texture mipmap.
- [updateTextureMappings(_:mode:regions:mipLevels:slices:numRegions:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemappings(_:mode:regions:miplevels:slices:numregions:)) — Encodes a command to update memory mappings for multiple regions inside a texture.
- [MTLSparseTextureMappingMode](https://developer.apple.com/documentation/metal/mtlsparsetexturemappingmode) — Options for sparse texture mapping.

### Updating texture memory assignments indirectly
- [updateTextureMapping(_:mode:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/updatetexturemapping(_:mode:indirectbuffer:indirectbufferoffset:)) — Encodes a command to update a texture’s memory mappings, specifying the parameters indirectly.

### Performing fence operations
- [update(_:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/update(_:)) — Encodes a command that instructs the GPU to update a fence, which signals passes waiting on the fence.
- [wait(for:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/wait(for:)) — Encodes a command that instructs the GPU to pause before starting the resource state commands until another pass updates a fence.

### Instance Methods
- [moveTextureMappings(sourceTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:destinationTexture:destinationSlice:destinationLevel:destinationOrigin:)](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder/movetexturemappings(sourcetexture:sourceslice:sourcelevel:sourceorigin:sourcesize:destinationtexture:destinationslice:destinationlevel:destinationorigin:))

## See also

### Sparse textures
- [Managing sparse texture memory](https://developer.apple.com/documentation/metal/managing-sparse-texture-memory) — Take direct control of memory allocation for texture data by using sparse textures.
- [Creating sparse heaps and sparse textures](https://developer.apple.com/documentation/metal/creating-sparse-heaps-and-sparse-textures) — Allocate memory for sparse textures by creating a sparse heap.
- [Converting between pixel regions and sparse tile regions](https://developer.apple.com/documentation/metal/converting-between-pixel-regions-and-sparse-tile-regions) — Learn how a sparse texture’s contents are organized in memory.
- [Assigning memory to sparse textures](https://developer.apple.com/documentation/metal/assigning-memory-to-sparse-textures) — Use a resource state encoder to allocate and deallocate sparse tiles for a sparse texture.
- [Reading and writing to sparse textures](https://developer.apple.com/documentation/metal/reading-and-writing-to-sparse-textures) — Decide how to handle access to unmapped texture regions.
- [Estimating how often a texture region is accessed](https://developer.apple.com/documentation/metal/estimating-how-often-a-texture-region-is-accessed) — Use texture access patterns to determine when you need to map a texture region.
- [MTLResourceStatePassDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepassdescriptor) — A configuration for a resource state pass, used to create a resource state command encoder.
- [MTLResourceStatePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor) — A description of where to store GPU counter information at the start and end of a resource state pass.
- [MTLResourceStatePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptorarray) — An array of sample buffer attachments for a resource state pass.
- [MTLMapIndirectArguments](https://developer.apple.com/documentation/metal/mtlmapindirectarguments) — The data layout for mapping sparse texture regions when using indirect commands.
