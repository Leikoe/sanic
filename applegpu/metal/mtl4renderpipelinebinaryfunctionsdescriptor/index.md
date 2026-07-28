# MTL4RenderPipelineBinaryFunctionsDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor>

Allows you to specify additional binary functions to link to each stage of a render pipeline.

## Declaration

```swift
class MTL4RenderPipelineBinaryFunctionsDescriptor
```

## Topics

### Instance Properties
- [fragmentAdditionalBinaryFunctions](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor/fragmentadditionalbinaryfunctions) — Provides an array of binary functions representing additional binary fragment shader functions.
- [meshAdditionalBinaryFunctions](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor/meshadditionalbinaryfunctions) — Provides an array of binary functions representing additional binary mesh shader functions.
- [objectAdditionalBinaryFunctions](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor/objectadditionalbinaryfunctions) — Provides an array of binary functions representing additional binary object shader functions.
- [tileAdditionalBinaryFunctions](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor/tileadditionalbinaryfunctions) — Provides an array of binary functions representing additional binary tile shader functions.
- [vertexAdditionalBinaryFunctions](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor/vertexadditionalbinaryfunctions) — Provides an array of binary functions representing additional binary vertex shader functions.

### Instance Methods
- [reset()](https://developer.apple.com/documentation/metal/mtl4renderpipelinebinaryfunctionsdescriptor/reset()) — Resets this descriptor to its default state.

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
- [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) — An object that configures new render pipeline state objects for tile shading.
- [MTLTileRenderPipelineColorAttachmentDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor) — A description of a tile-shading render pipeline’s color render target.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
