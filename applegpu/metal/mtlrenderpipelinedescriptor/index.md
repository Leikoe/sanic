# MTLRenderPipelineDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor>

An argument of options you pass to a GPU device to get a render pipeline state.

## Declaration

```swift
class MTLRenderPipelineDescriptor
```

## Overview

An [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) instance configures the state of the pipeline to use during a rendering pass, including rasterization (such as multisampling), visibility, blending, tessellation, and graphics function state. Use standard allocation and initialization techniques to create an [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) object. Then configure and use the descriptor to create an [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) object.

To specify the vertex or fragment function in the rendering pipeline descriptor, set the [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction) or [fragmentFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentfunction) property, respectively, to the desired [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) object. The system ignores the tessellation stage properties if you don’t set the [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction) property to a post-tessellation vertex function. A vertex function is a post-tessellation vertex function if the `[[ patch(patch-type, N) ]]` attribute precedes the function’s signature in your Metal Shading Language source. See the “Post-Tessellation Vertex Functions” section of [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) for more information.

Setting the [fragmentFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentfunction) property to `nil` disables the rasterization of pixels into the color attachment. This action is typically for outputting vertex function data into a buffer object, or for depth-only rendering.

If the vertex shader has an argument with per-vertex input attributes, set the [vertexDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexdescriptor) property to an [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) object that describes the organization of that vertex data.

### Multisampling and the render pipeline

If a color attachment supports multisampling (essentially, the attachment is an [MTLTextureType.type2DMultisample](https://developer.apple.com/documentation/metal/mtltexturetype/type2dmultisample) type color texture), you can create multiple samples per fragment, and the following rendering pipeline descriptor properties determine coverage:

- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount) is the number of samples for each pixel.

- If [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) is [true](https://developer.apple.com/documentation/Swift/true), the GPU uses the alpha channel fragment output for [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/colorattachments) to compute a coverage mask that affects the values the GPU writes to all attachments (color, depth, and stencil).

- If [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled) is [true](https://developer.apple.com/documentation/Swift/true), the GPU changes alpha channel fragment values for [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/colorattachments) to `1.0`, which is the largest representable value.

If [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) is [true](https://developer.apple.com/documentation/Swift/true), an implementation-defined `coverageToMask` function uses the alpha channel fragment output from [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/colorattachments) to create an intermediate coverage mask, which sets a number of bits in its output proportionally to the value of the floating-point input. For example, if the input is `0.0f`, the function sets the output to `0x0`. If the input is `1.0f`, the function sets all output bits (in effect, `~0x0`). If the input is `0.5f`, the function sets half of the bits, according to the implementation, which often uses dither patterns.

To determine a final coverage mask, the function performs a logical `AND` on the resulting coverage mask `alphaCoverageMask` with the masks from the rasterizer and fragment shader, as the following code shows:

```objective-c
if (alphaToCoverageEnabled) then
    alphaCoverageMask = coverageToMask(colorAttachment0.alpha);

finalCoverageMask = originalRasterizerCoverageMask
                    & alphaCoverageMask
                    & fragShaderSampleMaskOutput;
```

## Topics

### Identifying the render pipeline state object
- [label](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/label) — A string that identifies the render pipeline descriptor.

### Specifying graphics functions and associated data
- [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction) — The vertex function the pipeline calls to process vertices.
- [fragmentFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentfunction) — The fragment function the pipeline calls to process fragments.
- [maxVertexCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxvertexcallstackdepth) — The maximum function call depth from the top-most vertex shader function.
- [maxFragmentCallStackDepth](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxfragmentcallstackdepth) — The maximum function call depth from the top-most fragment shader function.

### Specifying buffer layouts and fetch behavior
- [vertexDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexdescriptor) — The organization of vertex data in an attribute’s argument table.

### Specifying buffer mutability
- [vertexBuffers](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexbuffers) — An array that contains the buffer mutability options for a render pipeline’s vertex function.
- [fragmentBuffers](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentbuffers) — An array that contains the buffer mutability options for a render pipeline’s fragment function.

### Specifying rendering pipeline state
- [reset()](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/reset()) — Specifies the default rendering pipeline state values for the descriptor.
- [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/colorattachments) — An array of attachments that store color data.
- [depthAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/depthattachmentpixelformat) — The pixel format of the attachment that stores depth data.
- [stencilAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/stencilattachmentpixelformat) — The pixel format of the attachment that stores stencil data.

### Specifying rasterization and visibility state
- [isAlphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatocoverageenabled) — A Boolean value that indicates whether to read and use the alpha channel fragment output for color attachments to compute a sample coverage mask.
- [isAlphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/isalphatooneenabled) — A Boolean value that indicates whether to force alpha channel values for color attachments to the largest representable value.
- [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled) — A Boolean value that determines whether the pipeline rasterizes primitives.
- [inputPrimitiveTopology](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/inputprimitivetopology) — The type of primitive topology the pipeline renders.
- [rasterSampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/rastersamplecount) — The number of samples the pipeline applies for each fragment.
- [MTLPrimitiveTopologyClass](https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass) — The primitive topologies available for rendering.
- [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/samplecount) — The number of samples the pipeline applies for each fragment.

### Specifying tessellation state
- [maxTessellationFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxtessellationfactor) — The maximum tessellation factor that the tessellator uses when tessellating patches.
- [isTessellationFactorScaleEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/istessellationfactorscaleenabled) — A Boolean value that determines whether the pipeline scales the tessellation factor.
- [tessellationFactorFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorformat) — The format of the tessellation factors in the tessellation factor buffer.
- [tessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationcontrolpointindextype) — The size of the control point indices in a control point index buffer.
- [tessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationfactorstepfunction) — The step function for determining the tessellation factors for a patch from the tessellation factor buffer.
- [tessellationOutputWindingOrder](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationoutputwindingorder) — The winding order of triangles from the tessellator.
- [tessellationPartitionMode](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/tessellationpartitionmode) — The partitioning mode that the tessellator uses to derive the number and spacing of segments for subdividing a corresponding edge.
- [MTLTessellationFactorFormat](https://developer.apple.com/documentation/metal/mtltessellationfactorformat) — Options for specifying the format of the tessellation factors in a tessellation factor buffer.
- [MTLTessellationControlPointIndexType](https://developer.apple.com/documentation/metal/mtltessellationcontrolpointindextype) — Options for specifying the size of the control point indices in a control point index buffer.
- [MTLTessellationFactorStepFunction](https://developer.apple.com/documentation/metal/mtltessellationfactorstepfunction) — Options for specifying the step function that determines the tessellation factors for a patch from the tessellation factor buffer.
- [MTLTessellationPartitionMode](https://developer.apple.com/documentation/metal/mtltessellationpartitionmode) — Options for choosing the partition mode that the tessellator applies when deriving the number and spacing of segments for subdividing a corresponding edge.

### Specifying indirect command buffers usage
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/supportindirectcommandbuffers) — A Boolean value that determines whether you can encode commands into an indirect command buffer using the render pipeline.

### Specifying the maximum vertex amplification count
- [maxVertexAmplificationCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/maxvertexamplificationcount) — The maximum vertex amplification count you can set when encoding render commands.

### Specifying precompiled shader binaries
- [supportAddingVertexBinaryFunctions](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/supportaddingvertexbinaryfunctions) — A Boolean value that indicates whether you can use the pipeline to create new pipelines by adding binary functions to the vertex shader’s callable functions list.
- [supportAddingFragmentBinaryFunctions](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/supportaddingfragmentbinaryfunctions) — A Boolean value that indicates whether you can use the pipeline to create new pipelines by adding binary functions to the fragment shader’s callable functions list.
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/binaryarchives) — An array of binary archives to search for precompiled versions of the shader.

### Specifying callable functions for the pipeline
- [vertexLinkedFunctions](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexlinkedfunctions) — Functions that you can specify as function arguments for the vertex shader when encoding commands that use the pipeline.
- [fragmentLinkedFunctions](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentlinkedfunctions) — Functions that you can specify as function arguments for the fragment shader when encoding commands that use the pipeline.

### Specifying shader validation
- [shaderValidation](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/shadervalidation) — A value that enables or disables shader validation for the pipeline.

### Instance Properties
- [fragmentPreloadedLibraries](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/fragmentpreloadedlibraries)
- [vertexPreloadedLibraries](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexpreloadedlibraries)

## See also

### Render pipeline states
- [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) — An interface that represents a graphics pipeline configuration for a render pass, which the pass applies to the draw commands you encode.
- [MTL4RenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor) — Groups together properties to create a render pipeline state object.
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
