# MTL4MeshRenderPipelineDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor>

Groups together properties you use to create a mesh render pipeline state object.

## Declaration

```swift
class MTL4MeshRenderPipelineDescriptor
```

## Overview

Compared to [MTLMeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor), this interface doesn’t offer a mechanism to hint to Metal mutability of object, mesh, or fragment buffers. Additionally, when you use this descriptor, you don’t specify binary archives.

## Topics

### Instance Properties
- [alphaToCoverageState](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/alphatocoveragestate) — Indicates whether to read and use the alpha channel fragment output of color attachments to compute a sample coverage mask.
- [alphaToOneState](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/alphatoonestate) — Indicates whether the pipeline forces alpha channel values of color attachments to the largest representable value.
- [colorAttachmentMappingState](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/colorattachmentmappingstate) — Sets the logical-to-physical rendering remap state.
- [colorAttachments](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/colorattachments) — Accesses an array containing descriptions of the color attachments this pipeline writes to.
- [fragmentFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/fragmentfunctiondescriptor) — Assigns a function descriptor representing the function this pipeline executes for each fragment.
- [fragmentStaticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/fragmentstaticlinkingdescriptor) — Provides static linking information for the fragment stage of the render pipeline.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/israsterizationenabled) — Determines whether the pipeline rasterizes primitives.
- [maxTotalThreadgroupsPerMeshGrid](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/maxtotalthreadgroupspermeshgrid) — Controls the largest number of threads the pipeline state can execute when the object stage of a mesh render pipeline you create from this descriptor dispatches its mesh stage.
- [maxTotalThreadsPerMeshThreadgroup](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/maxtotalthreadspermeshthreadgroup) — Controls the largest number of threads the pipeline state can execute in a single mesh shader threadgroup dispatch.
- [maxTotalThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/maxtotalthreadsperobjectthreadgroup) — Controls the largest number of threads the pipeline state can execute in a single object shader threadgroup dispatch.
- [maxVertexAmplificationCount](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/maxvertexamplificationcount) — Determines the maximum value that can you can pass as the pipeline’s amplification count.
- [meshFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/meshfunctiondescriptor) — Assigns a function descriptor representing the function this pipeline executes for each primitive in the mesh shader stage.
- [meshStaticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/meshstaticlinkingdescriptor) — Provides static linking information for the mesh stage of the render pipeline.
- [meshThreadgroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/meshthreadgroupsizeismultipleofthreadexecutionwidth) — Provides a guarantee to Metal regarding the number of threadgroup threads for the mesh stage of a pipeline you create from this descriptor.
- [objectFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/objectfunctiondescriptor) — Assigns a function descriptor representing the function this pipeline executes for each *object* in the object shader stage.
- [objectStaticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/objectstaticlinkingdescriptor) — Provides static linking information for the object stage of the render pipeline.
- [objectThreadgroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/objectthreadgroupsizeismultipleofthreadexecutionwidth) — Provides a guarantee to Metal regarding the number of threadgroup threads for the object stage of a pipeline you create from this descriptor.
- [payloadMemoryLength](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/payloadmemorylength) — Reserves storage for the object-to-mesh stage payload.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/rastersamplecount) — Sets number of samples this pipeline applies for each fragment.
- [requiredThreadsPerMeshThreadgroup](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/requiredthreadspermeshthreadgroup) — Controls the required number of mesh threads-per-threadgroup when drawing with a mesh shader pipeline you create from this descriptor.
- [requiredThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/requiredthreadsperobjectthreadgroup) — Controls the required number of object threads-per-threadgroup when drawing with a mesh shader pipeline you create from this descriptor.
- [supportFragmentBinaryLinking](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/supportfragmentbinarylinking) — Indicates whether you can use the render pipeline to create new pipelines by adding binary functions to the fragment shader function’s callable functions list.
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/supportindirectcommandbuffers) — Indicates whether the pipeline supports indirect command buffers.
- [supportMeshBinaryLinking](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/supportmeshbinarylinking) — Indicates whether you can use the render pipeline to create new pipelines by adding binary functions to the mesh shader function’s callable functions list.
- [supportObjectBinaryLinking](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/supportobjectbinarylinking) — Indicates whether you can use the render pipeline to create new pipelines by adding binary functions to the object shader function’s callable functions list.

### Instance Methods
- [reset()](https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/reset()) — Resets this descriptor to its default state.

## See also

### Render pipeline states
- [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) — An interface that represents a graphics pipeline configuration for a render pass, which the pass applies to the draw commands you encode.
- [MTL4RenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor) — Groups together properties to create a render pipeline state object.
- [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) — An argument of options you pass to a GPU device to get a render pipeline state.
- [MTLRenderPipelineFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinefunctionsdescriptor) — A collection of functions for updating a render pipeline.
- [MTLMeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor) — An object that configures new render pipeline state objects for mesh shading.
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
