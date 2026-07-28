# MTL4TileRenderPipelineDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor>

Groups together properties you use to create a tile render pipeline state object.

## Declaration

```swift
class MTL4TileRenderPipelineDescriptor
```

## Topics

### Instance Properties
- [colorAttachments](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/colorattachments) — Access an array of descriptors that configure the properties of each color attachment in the tile render pipeline.
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/maxtotalthreadsperthreadgroup) — Sets the maximum number of threads that the GPU can execute simultaneously within a single threadgroup in the tile render pipeline.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/rastersamplecount) — Configures the number of samples per pixel used for multisampling.
- [requiredThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/requiredthreadsperthreadgroup) — Sets the required number of threads per threadgroup for tile dispatches.
- [staticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/staticlinkingdescriptor) — Configures an object that contains information about functions to link to the tile render pipeline when Metal builds it.
- [supportBinaryLinking](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/supportbinarylinking) — Indicates whether the pipeline supports linking binary functions.
- [threadgroupSizeMatchesTileSize](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/threadgroupsizematchestilesize) — Indicating whether the size of the threadgroup matches the size of a tile in the render pipeline.
- [tileFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/tilefunctiondescriptor) — Configures the tile function that the render pipeline executes for each tile in the tile shader stage.

### Instance Methods
- [reset()](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor/reset()) — Resets the descriptor to the default state.

## See also

### Render pipeline states
- [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) — An interface that represents a graphics pipeline configuration for a render pass, which the pass applies to the draw commands you encode.
- [MTL4RenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor) — Groups together properties to create a render pipeline state object.
- [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) — An argument of options you pass to a GPU device to get a render pipeline state.
- [MTLRenderPipelineFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinefunctionsdescriptor) — A collection of functions for updating a render pipeline.
- [MTL4MeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor) — Groups together properties you use to create a mesh render pipeline state object.
- [MTLMeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor) — An object that configures new render pipeline state objects for mesh shading.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTL4RenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinecolorattachmentdescriptor)
- [MTLRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor) — A color render target that specifies the color configuration and color operations for a render pipeline.
- [MTLRenderPipelineColorAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray) — An array of render pipeline color attachment descriptor objects.
- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) — An object that configures new render pipeline state objects for tile shading.
- [MTLTileRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor) — A description of a tile-shading render pipeline’s color render target.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
- [MTL4RenderPipelineBinaryFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor) — Allows you to specify additional binary functions to link to each stage of a render pipeline.
