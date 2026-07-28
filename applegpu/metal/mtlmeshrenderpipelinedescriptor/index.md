# MTLMeshRenderPipelineDescriptor

*Class · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor>

An object that configures new render pipeline state objects for mesh shading.

## Declaration

```swift
class MTLMeshRenderPipelineDescriptor
```

## Topics

### Instance Properties
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/binaryarchives)
- [colorAttachments](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/colorattachments)
- [depthAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/depthattachmentpixelformat)
- [fragmentBuffers](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/fragmentbuffers)
- [fragmentFunction](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/fragmentfunction)
- [fragmentLinkedFunctions](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/fragmentlinkedfunctions)
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/isalphatocoverageenabled)
- [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/isalphatooneenabled)
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/israsterizationenabled)
- [label](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/label)
- [maxTotalThreadgroupsPerMeshGrid](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/maxtotalthreadgroupspermeshgrid)
- [maxTotalThreadsPerMeshThreadgroup](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/maxtotalthreadspermeshthreadgroup)
- [maxTotalThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/maxtotalthreadsperobjectthreadgroup)
- [maxVertexAmplificationCount](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/maxvertexamplificationcount)
- [meshBuffers](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/meshbuffers)
- [meshFunction](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/meshfunction)
- [meshLinkedFunctions](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/meshlinkedfunctions)
- [meshThreadgroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/meshthreadgroupsizeismultipleofthreadexecutionwidth)
- [objectBuffers](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/objectbuffers)
- [objectFunction](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/objectfunction)
- [objectLinkedFunctions](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/objectlinkedfunctions)
- [objectThreadgroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/objectthreadgroupsizeismultipleofthreadexecutionwidth)
- [payloadMemoryLength](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/payloadmemorylength)
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/rastersamplecount)
- [requiredThreadsPerMeshThreadgroup](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/requiredthreadspermeshthreadgroup)
- [requiredThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/requiredthreadsperobjectthreadgroup)
- [shaderValidation](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/shadervalidation) — A value that enables or disables shader validation for the pipeline.
- [stencilAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/stencilattachmentpixelformat)
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/supportindirectcommandbuffers)

### Instance Methods
- [reset()](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor/reset())

## See also

### Render pipeline states
- [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) — An interface that represents a graphics pipeline configuration for a render pass, which the pass applies to the draw commands you encode.
- [MTL4RenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor) — Groups together properties to create a render pipeline state object.
- [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) — An argument of options you pass to a GPU device to get a render pipeline state.
- [MTLRenderPipelineFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinefunctionsdescriptor) — A collection of functions for updating a render pipeline.
- [MTL4MeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor) — Groups together properties you use to create a mesh render pipeline state object.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTL4RenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinecolorattachmentdescriptor)
- [MTLRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor) — A color render target that specifies the color configuration and color operations for a render pipeline.
- [MTLRenderPipelineColorAttachmentDescriptorArray](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray) — An array of render pipeline color attachment descriptor objects.
- [MTL4TileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor) — Groups together properties you use to create a tile render pipeline state object.
- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) — An object that configures new render pipeline state objects for tile shading.
- [MTLTileRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor) — A description of a tile-shading render pipeline’s color render target.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
- [MTL4RenderPipelineBinaryFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor) — Allows you to specify additional binary functions to link to each stage of a render pipeline.
