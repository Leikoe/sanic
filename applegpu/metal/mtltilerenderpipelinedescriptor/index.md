# MTLTileRenderPipelineDescriptor

*Class · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor>

An object that configures new render pipeline state objects for tile shading.

## Declaration

```swift
class MTLTileRenderPipelineDescriptor
```

## Topics

### Identifying the render pipeline
- [label](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/label) — A string that identifies the tile pipeline descriptor.

### Specifying graphics functions and associated data
- [tileFunction](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/tilefunction) — The compute kernel or fragment function the pipeline calls.
- [tileBuffers](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/tilebuffers) — An array that contains the buffer mutability options for a render pipeline’s tile function.
- [maxCallStackDepth](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/maxcallstackdepth) — The maximum call stack depth for indirect function calls in tile shaders.

### Specifying rasterization and visibility state
- [threadgroupSizeMatchesTileSize](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/threadgroupsizematchestilesize) — A Boolean value that indicates whether all threadgroups for this pipeline completely cover tiles.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/rastersamplecount) — The number of samples in each fragment.

### Specifying rendering pipeline state
- [reset()](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/reset()) — Specifies the default rendering pipeline state values for the descriptor.
- [colorAttachments](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/colorattachments) — An array of attachments that store color data.

### Specifying threads per threadgroup
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/maxtotalthreadsperthreadgroup) — The maximum number of threads in a threadgroup when dispatching a command using the pipeline.

### Specifying precompiled shader binaries
- [supportAddingBinaryFunctions](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/supportaddingbinaryfunctions) — A Boolean value that indicates whether you can use the pipeline to create new pipelines by adding binary functions to its callable functions list.
- [binaryArchives](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/binaryarchives) — An array of binary archives to search for precompiled versions of the shader.

### Specifying callable functions for the pipeline
- [linkedFunctions](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/linkedfunctions) — Functions that you can specify as function arguments for the tile shader when encoding commands that use the pipeline.

### Specifying shader validation
- [shaderValidation](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/shadervalidation) — A value that enables or disables shader validation for the pipeline.

### Instance Properties
- [preloadedLibraries](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/preloadedlibraries)
- [requiredThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/requiredthreadsperthreadgroup)

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
- [MTL4TileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor) — Groups together properties you use to create a tile render pipeline state object.
- [MTLTileRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor) — A description of a tile-shading render pipeline’s color render target.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
- [MTL4RenderPipelineBinaryFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor) — Allows you to specify additional binary functions to link to each stage of a render pipeline.
