# Resource creation

*API Collection*

<https://developer.apple.com/documentation/metal/resource-creation>

Load assets with input/output queues and make various resource instances, such as buffers, textures, acceleration structures, and memory heaps.

## Topics

### Working with resource heaps
- [makeHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)) — Creates a new GPU heap instance.
- [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)) — Returns the size and alignment, in bytes, of a buffer if you create it from a heap.
- [heapTextureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heaptexturesizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of a texture if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(size:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(size:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap with a descriptor.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.

### Creating buffers
- [maxBufferLength](https://developer.apple.com/documentation/metal/mtldevice/maxbufferlength) — The largest amount of memory, in bytes, that a GPU device can allocate to a buffer instance.
- [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:)) — Creates a buffer the method clears with zero values.
- [makeBuffer(bytes:length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytes:length:options:)) — Allocates a new buffer of a given length and initializes its contents by copying existing data into it.
- [makeBuffer(bytesNoCopy:length:options:deallocator:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytesnocopy:length:options:deallocator:)) — Creates a buffer that wraps an existing contiguous memory allocation.

### Creating textures
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:)) — Creates a new texture instance.
- [makeTexture(descriptor:iosurface:plane:)](https://developer.apple.com/documentation/metal/mtldevice/maketexture(descriptor:iosurface:plane:)) — Creates a texture instance that uses I/O surface to store its underlying data.
- [makeSharedTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(descriptor:)) — Creates a texture that you can share across process boundaries.
- [makeSharedTexture(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedtexture(handle:)) — Creates a texture that references a shared texture.
- [minimumLinearTextureAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumlineartexturealignment(for:)) — Returns the minimum alignment the GPU device requires to create a linear texture from a buffer.
- [minimumTextureBufferAlignment(for:)](https://developer.apple.com/documentation/metal/mtldevice/minimumtexturebufferalignment(for:)) — Returns the minimum alignment the GPU device requires to create a texture buffer from a buffer.

### Creating samplers
- [supportsTextureSampleCount(_:)](https://developer.apple.com/documentation/metal/mtldevice/supportstexturesamplecount(_:)) — Returns a Boolean value that indicates whether the GPU can sample a texture with a specific number of sample points.
- [makeSamplerState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makesamplerstate(descriptor:)) — Creates a sampler state instance.
- [getDefaultSamplePositions(sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/getdefaultsamplepositions(samplecount:)) — Returns the default sample locations based on the number of samples.

### Working with sparse textures
- [sparseTileSize(textureType:pixelFormat:sampleCount:sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(texturetype:pixelformat:samplecount:sparsepagesize:)) — Returns the dimensions of a sparse tile for a texture that has a specific sparse page size.
- [sparseTileSize(with:pixelFormat:sampleCount:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesize(with:pixelformat:samplecount:)) — Returns the dimensions of a sparse tile for a texture.
- [sparseTileSizeInBytes(sparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes(sparsepagesize:)) — Returns the size, in bytes, of a sparse tile the GPU device creates with a specific page size.
- [sparseTileSizeInBytes](https://developer.apple.com/documentation/metal/mtldevice/sparsetilesizeinbytes) — Returns the size, in bytes, of a sparse tile the GPU device creates using a default page size.
- [convertSparsePixelRegions(_:toTileRegions:withTileSize:alignmentMode:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsepixelregions(_:totileregions:withtilesize:alignmentmode:numregions:)) — Converts a list of sparse pixel regions to tile regions.
- [convertSparseTileRegions(_:toPixelRegions:withTileSize:numRegions:)](https://developer.apple.com/documentation/metal/mtldevice/convertsparsetileregions(_:topixelregions:withtilesize:numregions:)) — Converts a list of sparse tile regions to pixel regions.
- [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) — The page size options, in kilobytes, for sparse textures.
- [MTLSparseTextureRegionAlignmentMode](https://developer.apple.com/documentation/metal/mtlsparsetextureregionalignmentmode) — Options used when converting between a pixel-based region within a texture to a tile-based region.

### Creating acceleration structures for ray tracing
- [makeAccelerationStructure(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(descriptor:)) — Creates a new ray-tracing acceleration structure from a descriptor.
- [makeAccelerationStructure(size:)](https://developer.apple.com/documentation/metal/mtldevice/makeaccelerationstructure(size:)) — Creates a new acceleration structure with a specific size.
- [accelerationStructureSizes(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/accelerationstructuresizes(descriptor:)) — Returns the buffer sizes the GPU device needs to build, refit, and store an acceleration structure.
- [MTLAccelerationStructureSizes](https://developer.apple.com/documentation/metal/mtlaccelerationstructuresizes) — The expected sizes for a ray-tracing acceleration structure.

### Creating argument buffer encoders
- [argumentBuffersSupport](https://developer.apple.com/documentation/metal/mtldevice/argumentbufferssupport) — Returns the GPU device’s support tier for argument buffers.
- [maxArgumentBufferSamplerCount](https://developer.apple.com/documentation/metal/mtldevice/maxargumentbuffersamplercount) — The maximum number of unique argument buffer samplers per app.
- [makeArgumentEncoder(arguments:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)) — Creates a new argument encoder for an array of arguments.
- [makeArgumentEncoder(bufferBinding:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(bufferbinding:)) — Creates a new argument encoder for a buffer binding.

### Creating fences and events
- [makeFence()](https://developer.apple.com/documentation/metal/mtldevice/makefence()) — Creates a new memory fence instance.
- [makeEvent()](https://developer.apple.com/documentation/metal/mtldevice/makeevent()) — Creates a new event instance that you can use to synchronize commands and resources within the same GPU device.
- [makeSharedEvent()](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent()) — Creates a new shared event instance that you can use to synchronize commands and resources across different GPU devices.
- [makeSharedEvent(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent(handle:)) — Recreates a shared event from a handle.

### Creating rasterization rate maps
- [supportsRasterizationRateMap(layerCount:)](https://developer.apple.com/documentation/metal/mtldevice/supportsrasterizationratemap(layercount:)) — Returns a Boolean value that indicates whether the GPU can create a rasterization rate map with a specific number of layers.
- [makeRasterizationRateMap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerasterizationratemap(descriptor:)) — Creates a rasterization rate map instance.

## See also

### Working with GPU devices
- [Device inspection](https://developer.apple.com/documentation/metal/device-inspection) — Locate and identify a GPU and the features it supports, and sample its counters.
- [Work submission](https://developer.apple.com/documentation/metal/work-submission) — Create queues that submit work to the GPU or load assets into GPU resources, and indirect command buffers that group your frequent commands together.
- [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation) — Create pipeline states for render and compute passes, samplers, depth and stencil states, and indirect command buffers.
- [Shader library and archive creation](https://developer.apple.com/documentation/metal/shader-library-and-archive-creation) — Create static and dynamic shader libraries, and binary shader archives.
