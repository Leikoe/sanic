# Managing sparse texture memory

*Article*

<https://developer.apple.com/documentation/metal/managing-sparse-texture-memory>

Take direct control of memory allocation for texture data by using sparse textures.

## Overview

When you create a texture, by default, Metal allocates memory to hold the texture’s pixel data. In some cases, such as when you’re implementing texture streaming, you generally use only a fraction of this memory. When you use sparse textures, you take ownership of memory management for texture data and decide when to allocate and deallocate memory for a texture. In this way, sparse textures help you to use memory more efficiently.

To use sparse textures, allocate a sparse heap from which to allocate memory, and then create sparse textures from this heap. Initially, a texture has no storage. To add storage to a region inside the texture, ask the GPU to map memory from the heap for that region. A *sparse tile* is a memory allocation (as opposed to memory tiles a GPU uses for tile-based rendering). Sparse tiles are conceptually similar to virtual memory pages. When you don’t need a region to have storage, you can unmap its sparse tile and recover that memory.

![image](https://docs-assets.developer.apple.com/published/2463ddd14ea55eb4285774acb6f90d69/managing-sparse-texture-memory-1%402x.png)

Because sparse textures work closely with texture mipmaps, you should be familiar with mipmaps before using sparse textures. For more information, see [Improving texture sampling quality and performance with mipmaps](https://developer.apple.com/documentation/metal/improving-texture-sampling-quality-and-performance-with-mipmaps).

### Check for sparse texture support

Not all GPUs support sparse textures. Check for support on the device object before attempting to use sparse textures:

```objective-c
- (Boolean) supportsSparseTextures
{
    return [_device supportsFamily: MTLGPUFamilyApple6 ];
}
```

## See also

### Sparse textures
- [Creating sparse heaps and sparse textures](https://developer.apple.com/documentation/metal/creating-sparse-heaps-and-sparse-textures) — Allocate memory for sparse textures by creating a sparse heap.
- [Converting between pixel regions and sparse tile regions](https://developer.apple.com/documentation/metal/converting-between-pixel-regions-and-sparse-tile-regions) — Learn how a sparse texture’s contents are organized in memory.
- [Assigning memory to sparse textures](https://developer.apple.com/documentation/metal/assigning-memory-to-sparse-textures) — Use a resource state encoder to allocate and deallocate sparse tiles for a sparse texture.
- [Reading and writing to sparse textures](https://developer.apple.com/documentation/metal/reading-and-writing-to-sparse-textures) — Decide how to handle access to unmapped texture regions.
- [Estimating how often a texture region is accessed](https://developer.apple.com/documentation/metal/estimating-how-often-a-texture-region-is-accessed) — Use texture access patterns to determine when you need to map a texture region.
- [MTLResourceStatePassDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepassdescriptor) — A configuration for a resource state pass, used to create a resource state command encoder.
- [MTLResourceStatePassSampleBufferAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor) — A description of where to store GPU counter information at the start and end of a resource state pass.
- [MTLResourceStatePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptorarray) — An array of sample buffer attachments for a resource state pass.
- [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) — An encoder that encodes commands that modify resource configurations.
- [MTLMapIndirectArguments](https://developer.apple.com/documentation/metal/mtlmapindirectarguments) — The data layout for mapping sparse texture regions when using indirect commands.
