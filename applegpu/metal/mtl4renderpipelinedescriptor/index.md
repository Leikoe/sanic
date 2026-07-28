# MTL4RenderPipelineDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor>

Groups together properties to create a render pipeline state object.

## Declaration

```swift
class MTL4RenderPipelineDescriptor
```

## Overview

Compared to [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor), this interface doesn’t offer a mechanism to hint to Metal mutability of vertex and fragment buffers. Additionally, using this descriptor, you don’t specify binary archives.

## Topics

### Instance Properties
- [alphaToCoverageState](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/alphatocoveragestate) — Indicates whether to read and use the alpha channel fragment output of color attachments to compute a sample coverage mask.
- [alphaToOneState](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/alphatoonestate) — Indicates whether the pipeline forces alpha channel values of color attachments to the largest representable value.
- [colorAttachmentMappingState](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/colorattachmentmappingstate) — Configures a logical-to-physical rendering remap state.
- [colorAttachments](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/colorattachments) — Accesses an array containing descriptions of the color attachments this pipeline writes to.
- [fragmentFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/fragmentfunctiondescriptor) — Assigns the shader function that this pipeline executes for each fragment.
- [fragmentStaticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/fragmentstaticlinkingdescriptor) — Provides static linking information for the fragment stage of the render pipeline.
- [inputPrimitiveTopology](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/inputprimitivetopology) — Assigns type of primitive topology this pipeline renders.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/israsterizationenabled) — Determines whether the pipeline rasterizes primitives.
- [maxVertexAmplificationCount](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/maxvertexamplificationcount) — Determines the maximum value that can you can pass as the pipeline’s amplification count.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/rastersamplecount) — Controls the number of samples this pipeline applies for each fragment.
- [supportFragmentBinaryLinking](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/supportfragmentbinarylinking) — Indicates whether you can use the pipeline to create new pipelines by adding binary functions to the fragment shader function’s callable functions list.
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/supportindirectcommandbuffers) — Indicates whether the pipeline supports indirect command buffers.
- [supportVertexBinaryLinking](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/supportvertexbinarylinking) — Indicates whether you can use the render pipeline to create new pipelines by adding binary functions to the vertex shader function’s callable functions list.
- [vertexDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/vertexdescriptor) — Configures an optional vertex descriptor for the vertex input.
- [vertexFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/vertexfunctiondescriptor) — Assigns the shader function that this pipeline executes for each vertex.
- [vertexStaticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/vertexstaticlinkingdescriptor) — Provides static linking information for the vertex stage of the render pipeline.

### Instance Methods
- [reset()](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/reset()) — Resets this descriptor to its default state.

## See also

### Render pipeline states
- [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) — An interface that represents a graphics pipeline configuration for a render pass, which the pass applies to the draw commands you encode.
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
- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) — An object that configures new render pipeline state objects for tile shading.
- [MTLTileRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor) — A description of a tile-shading render pipeline’s color render target.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
- [MTL4RenderPipelineBinaryFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor) — Allows you to specify additional binary functions to link to each stage of a render pipeline.
