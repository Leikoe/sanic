# MTLRenderPipelineState

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate>

An interface that represents a graphics pipeline configuration for a render pass, which the pass applies to the draw commands you encode.

## Declaration

```swift
protocol MTLRenderPipelineState : MTLAllocation, Sendable
```

## Overview

The [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) protocol is an interface that represents a specific configuration for the graphics-rendering pipeline, including which shaders it uses. Use a pipeline state to configure a render pass by calling the [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) method of an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance.

To create a pipeline state, call the appropriate [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) method (see [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation)). You typically make pipeline states at a noncritical time, like when the app first launches. This is because graphics drivers may need time to evaluate and build each pipeline state. However, you can quickly use and reuse each pipeline state throughout your app’s lifetime.

## Topics

### Identifying a pipeline state
- [device](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/device) — The device instance that creates the pipeline state.
- [label](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/label) — A string that helps you identify the render pipeline state during debugging.
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/gpuresourceid) — An unique identifier that represents the pipeline state, which you can add to an argument buffer.

### Checking object shader memory requirements
- [maxTotalThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/maxtotalthreadsperobjectthreadgroup) — The largest number of threads the pipeline state can have in a single object shader threadgroup.
- [objectThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/objectthreadexecutionwidth) — The number of threads the render pass applies to a SIMD group for an object shader.

### Checking mesh shader memory requirements
- [maxTotalThreadsPerMeshThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/maxtotalthreadspermeshthreadgroup) — The largest number of threads the pipeline state can have in a single mesh shader threadgroup.
- [maxTotalThreadgroupsPerMeshGrid](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/maxtotalthreadgroupspermeshgrid) — The largest number of threadgroups the pipeline state can have in a single mesh shader grid.
- [meshThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/meshthreadexecutionwidth) — The number of threads the render pass applies to a SIMD group for a mesh shader.

### Checking tile shader memory requirements
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/maxtotalthreadsperthreadgroup) — The largest number of threads the pipeline state can have in a single tile shader threadgroup.
- [threadgroupSizeMatchesTileSize](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/threadgroupsizematchestilesize) — A Boolean value that indicates whether the pipeline state needs a threadgroup’s size to equal a tile’s size.
- [imageblockSampleLength](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/imageblocksamplelength) — The memory size, in byes, of the render pipeline’s imageblock for a single sample.
- [imageblockMemoryLength(forDimensions:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/imageblockmemorylength(fordimensions:)) — Returns the length of an imageblock’s memory for the specified imageblock dimensions.

### Checking feature support
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/supportindirectcommandbuffers) — A Boolean value that indicates whether the render pipeline supports encoding commands into an indirect command buffer.

### Checking shader validation
- [shaderValidation](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/shadervalidation) — The current state of shader validation for the pipeline.

### Creating function handles and tables
- [functionHandle(function:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/functionhandle(function:stage:)-7uvul) — Creates a function handle for a shader.
- [makeVisibleFunctionTable(descriptor:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makevisiblefunctiontable(descriptor:stage:)) — Creates a new visible function table.
- [makeIntersectionFunctionTable(descriptor:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makeintersectionfunctiontable(descriptor:stage:)) — Creates a new intersection function table.

### Creating modified clones of the render pipeline
- [makeRenderPipelineState(additionalBinaryFunctions:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makerenderpipelinestate(additionalbinaryfunctions:)-84te1) — Creates a new pipeline state that’s a copy of the current pipeline state with additional shaders.

### Instance Properties
- [reflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/reflection) — The render pipeline’s reflection information, if available.
- [requiredThreadsPerMeshThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/requiredthreadspermeshthreadgroup)
- [requiredThreadsPerObjectThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/requiredthreadsperobjectthreadgroup)
- [requiredThreadsPerTileThreadgroup](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/requiredthreadspertilethreadgroup)

### Instance Methods
- [functionHandle(function:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/functionhandle(function:stage:)-1pgxo) — Obtains the function handle for a specific function this pipeline state links at the binary level.
- [functionHandle(withName:stage:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/functionhandle(withname:stage:)) — Obtains a function handle for the a specific function this pipeline links at the Metal IR level.
- [makeRenderPipelineDescriptorForSpecialization()](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makerenderpipelinedescriptorforspecialization()) — Creates a render pipeline descriptor from this pipeline that you can use for pipeline specialization.
- [makeRenderPipelineState(additionalBinaryFunctions:)](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makerenderpipelinestate(additionalbinaryfunctions:)-49r1w) — Creates a new render pipeline state by adding binary functions to each stage of this pipeline state.

## See also

### Render pipeline states
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
- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) — An object that configures new render pipeline state objects for tile shading.
- [MTLTileRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor) — A description of a tile-shading render pipeline’s color render target.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
- [MTL4RenderPipelineBinaryFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor) — Allows you to specify additional binary functions to link to each stage of a render pipeline.
