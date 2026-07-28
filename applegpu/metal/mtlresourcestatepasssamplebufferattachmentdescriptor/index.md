# MTLResourceStatePassSampleBufferAttachmentDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor>

A description of where to store GPU counter information at the start and end of a resource state pass.

## Declaration

```swift
class MTLResourceStatePassSampleBufferAttachmentDescriptor
```

## Topics

### Configuring the sample buffer attachment
- [sampleBuffer](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/samplebuffer) — A specialized memory buffer that the GPU uses to store its counter data during the resource state pass.
- [startOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/startofencodersampleindex) — The index the Metal device object should use to store GPU counters when starting the resource state pass.
- [endOfEncoderSampleIndex](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptor/endofencodersampleindex) — The index the Metal device object should use to store GPU counters when ending the resource state pass.

## See also

### Sparse textures
- [Managing sparse texture memory](https://developer.apple.com/documentation/metal/managing-sparse-texture-memory) — Take direct control of memory allocation for texture data by using sparse textures.
- [Creating sparse heaps and sparse textures](https://developer.apple.com/documentation/metal/creating-sparse-heaps-and-sparse-textures) — Allocate memory for sparse textures by creating a sparse heap.
- [Converting between pixel regions and sparse tile regions](https://developer.apple.com/documentation/metal/converting-between-pixel-regions-and-sparse-tile-regions) — Learn how a sparse texture’s contents are organized in memory.
- [Assigning memory to sparse textures](https://developer.apple.com/documentation/metal/assigning-memory-to-sparse-textures) — Use a resource state encoder to allocate and deallocate sparse tiles for a sparse texture.
- [Reading and writing to sparse textures](https://developer.apple.com/documentation/metal/reading-and-writing-to-sparse-textures) — Decide how to handle access to unmapped texture regions.
- [Estimating how often a texture region is accessed](https://developer.apple.com/documentation/metal/estimating-how-often-a-texture-region-is-accessed) — Use texture access patterns to determine when you need to map a texture region.
- [MTLResourceStatePassDescriptor](https://developer.apple.com/documentation/metal/mtlresourcestatepassdescriptor) — A configuration for a resource state pass, used to create a resource state command encoder.
- [MTLResourceStatePassSampleBufferAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlresourcestatepasssamplebufferattachmentdescriptorarray) — An array of sample buffer attachments for a resource state pass.
- [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) — An encoder that encodes commands that modify resource configurations.
- [MTLMapIndirectArguments](https://developer.apple.com/documentation/metal/mtlmapindirectarguments) — The data layout for mapping sparse texture regions when using indirect commands.
