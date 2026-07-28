# MTLMapIndirectArguments

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlmapindirectarguments>

The data layout for mapping sparse texture regions when using indirect commands.

## Declaration

```swift
struct MTLMapIndirectArguments
```

## Topics

### Creating indirect mapping arguments
- [init()](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/init()) — Returns a default data layout for mapping sparse texture regions.
- [init(regionOriginX:regionOriginY:regionOriginZ:regionSizeWidth:regionSizeHeight:regionSizeDepth:mipMapLevel:sliceId:)](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/init(regionoriginx:regionoriginy:regionoriginz:regionsizewidth:regionsizeheight:regionsizedepth:mipmaplevel:sliceid:)) — Returns a new data layout for mapping sparse texture regions.

### Specifying region origin
- [regionOriginX](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/regionoriginx) — The x coordinate of the region to change, measured in tile coordinates.
- [regionOriginY](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/regionoriginy) — The y coordinate of the region to change, measured in tile coordinates.
- [regionOriginZ](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/regionoriginz) — The z coordinate of the region to change, measured in tile coordinates.

### Specifying region dimensions
- [regionSizeWidth](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/regionsizewidth) — The width of the region, measured in tile coordinates.
- [regionSizeHeight](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/regionsizeheight) — The height of the region, measured in tile coordinates.
- [regionSizeDepth](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/regionsizedepth) — The depth of the region, measured in tile coordinates.

### Specifying texture location
- [mipMapLevel](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/mipmaplevel) — The mipmap to change.
- [sliceId](https://developer.apple.com/documentation/metal/mtlmapindirectarguments/sliceid) — The texture slice to change.

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
- [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder) — An encoder that encodes commands that modify resource configurations.
