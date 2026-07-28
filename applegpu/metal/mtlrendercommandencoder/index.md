# MTLRenderCommandEncoder

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder>

Encodes configuration and draw commands for a single render pass into a command buffer.

## Declaration

```swift
protocol MTLRenderCommandEncoder : MTLCommandEncoder
```

## Overview

A render pass draws a scene, or a component within a scene, to its render *attachments*, the outputs of a render pass. You can render to those outputs with various approaches, including techniques that apply the following:

- Primitive drawing

- Mesh drawing

- Ray tracing

- Dispatching tile shaders

To create an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance, call the [makeRenderCommandEncoder(descriptor:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makerendercommandencoder(descriptor:)) method of an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance, or the [makeRenderCommandEncoder()](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder/makerendercommandencoder()) method of an [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) instance.

To configure the render pass for your first drawing commands, start with a pipeline state by passing an [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) instance to the encoder’s [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) method. You create the pipeline states your render pass needs, typically ahead of time, by calling one or more [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods (see [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation)).

> **Tip:**
>  Avoid visual stutter by creating pipeline states at a noncritical time, such as during launch, because of the time it can take to make them.

Configure other encoder settings by calling the methods on the [Render pass configuration](https://developer.apple.com/documentation/metal/render-pass-configuration) page. For example, you may need to configure the pass’s viewport, its scissor rectangle, and the settings for depth and stencil tests.

Assign resources, such as buffers and textures, for the shaders that depend on them. For more information, see the shader-specific pages in the resource preparation section, such as [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) and [Fragment shader resource preparation commands](https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands). If your shaders access resources through an argument buffer, make those resources *resident* in GPU memory by calling the methods on the [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands) page.

Encode drawing commands after you configure the state and resources the commands depend on. The encoder maintains its current state and applies it to all subsequent draw commands. For drawing commands that need different states or resources, reconfigure the render pass appropriately and then encode those draw commands. Repeat the process for each batch of drawing commands that depend on the same render pass configuration and resources.

When you finish encoding the render pass’s commands, finalize it into the command buffer by calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method.

### Command stages

Most render commands apply to one or more stages within a pass. The following table shows which stages apply to each command:

| Function | MTLStages |
|---|---|
| [drawPrimitives(type:vertexStart:vertexCount:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:vertexstart:vertexcount:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPrimitives(type:vertexStart:vertexCount:instanceCount:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:vertexstart:vertexcount:instancecount:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPrimitives(type:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:vertexstart:vertexcount:instancecount:baseinstance:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPrimitives(type:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:indirectbuffer:indirectbufferoffset:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:basevertex:baseinstance:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(type:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indextype:indexbuffer:indexbufferoffset:indirectbuffer:indirectbufferoffset:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawMeshThreads(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreads(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) | [object](https://developer.apple.com/documentation/metal/mtlstages/object)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawMeshThreadgroups(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) | [object](https://developer.apple.com/documentation/metal/mtlstages/object)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawMeshThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(indirectbuffer:indirectbufferoffset:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) | [object](https://developer.apple.com/documentation/metal/mtlstages/object)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPatches(numberOfPatchControlPoints:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawpatches(numberofpatchcontrolpoints:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:instancecount:baseinstance:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:indirectbuffer:indirectbufferoffset:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPatches(numberOfPatchControlPoints:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:instancecount:baseinstance:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:indirectbuffer:indirectbufferoffset:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [dispatchThreadsPerTile(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/dispatchthreadspertile(_:)) | [tile](https://developer.apple.com/documentation/metal/mtlstages/tile) |
| [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:range:))![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[executeCommandsInBuffer:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer:withrange:) | None |
| [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:))![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer:indirectbuffer:indirectbufferoffset:) | None |
| [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) | None |

Draw commands don’t apply to [fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) when the [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) for the draw disables rasterization. See [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/israsterizationenabled).

Mesh draw commands don’t apply to [object](https://developer.apple.com/documentation/metal/mtlstages/object) when the [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) for the draw doesn’t have an object shader.

The [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:range:)) and [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) commands don’t apply to any stage, which means you can’t use a barrier to wait for all commands in an indirect command buffer to complete. However, each command within the [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) applies to the same stages as when you encode the equivalent command directly.

> **Note:**
> [MTLRenderStages](https://developer.apple.com/documentation/metal/mtlrenderstages) and its values have the same functionality as [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and its corresponding stage values.

For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Configuration commands
- [Render pass configuration](https://developer.apple.com/documentation/metal/render-pass-configuration) — Set a render pass’s pipeline state, attachment actions, viewports, and so on, that affect subsequent drawing commands.

### Resource preparation commands
- [Mesh and object shader resource preparation commands](https://developer.apple.com/documentation/metal/mesh-and-object-shader-resource-preparation-commands) — Assign resources to mesh and object shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) — Assign resources to vertex shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Fragment shader resource preparation commands](https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands) — Assign resources to fragment shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Tile shaders resource preparation commands](https://developer.apple.com/documentation/metal/tile-shaders-resource-preparation-commands) — Assign resources to tile shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands) — Load individual resources and multiple resources within a heap into GPU memory so that they’re available to shaders through argument buffers.

### Drawing with vertices
- [drawPrimitives(type:vertexStart:vertexCount:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:vertexstart:vertexcount:)) — Encodes a draw command that renders an instance of a geometric primitive.
- [drawPrimitives(type:vertexStart:vertexCount:instanceCount:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:vertexstart:vertexcount:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive.
- [drawPrimitives(type:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:vertexstart:vertexcount:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive that starts with a custom instance identification number.
- [drawPrimitives(type:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawprimitives(type:indirectbuffer:indirectbufferoffset:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indirect arguments.

### Drawing with indexed vertices
- [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:)) — Encodes a draw command that renders an instance of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(type:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indexcount:indextype:indexbuffer:indexbufferoffset:instancecount:basevertex:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices, starting with a custom vertex and instance.
- [drawIndexedPrimitives(type:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedprimitives(type:indextype:indexbuffer:indexbufferoffset:indirectbuffer:indirectbufferoffset:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices and indirect arguments.

### Drawing with meshes
- [drawMeshThreads(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreads(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threads.
- [drawMeshThreadgroups(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threadgroups.
- [drawMeshThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(indirectbuffer:indirectbufferoffset:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with indirect arguments.

### Drawing with tessellation patches
- [drawPatches(numberOfPatchControlPoints:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawpatches(numberofpatchcontrolpoints:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of tessellated patches.
- [drawPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:indirectbuffer:indirectbufferoffset:)) — Encodes a draw command that renders multiple instances of tessellated patches with indirect arguments.

### Drawing with indexed tessellation patches
- [drawIndexedPatches(numberOfPatchControlPoints:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchstart:patchcount:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of tessellated patches with a control point index buffer.
- [drawIndexedPatches(numberOfPatchControlPoints:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawindexedpatches(numberofpatchcontrolpoints:patchindexbuffer:patchindexbufferoffset:controlpointindexbuffer:controlpointindexbufferoffset:indirectbuffer:indirectbufferoffset:)) — Encodes a draw command that renders multiple instances of tessellated patches with a control point index buffer and indirect arguments.

### Drawing with tile shaders
- [dispatchThreadsPerTile(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/dispatchthreadspertile(_:)) — Encodes a command that invokes GPU functions from the encoder’s current tile render pipeline state.
- [tileWidth](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tilewidth) — The width of the tiles, in pixels, for the render command encoder.
- [tileHeight](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/tileheight) — The height of the tiles, in pixels, for the render command encoder.

### Preventing resource access conflicts
- [waitForFence(_:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/waitforfence(_:before:)) — Encodes a command that instructs the GPU to pause before starting one or more stages of the render pass until a pass updates a fence.
- [updateFence(_:after:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/updatefence(_:after:)) — Encodes a command that instructs the GPU to update a fence after one or more stages, which can unblock other passes waiting for the fence.
- [memoryBarrier(resources:after:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(resources:after:before:)) — Creates a memory barrier that enforces the order of write and read operations for specific resources.
- [memoryBarrier(scope:after:before:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/memorybarrier(scope:after:before:)) — Creates a memory barrier that enforces the order of write and read operations for specific resource types.

### Running commands from indirect command buffers
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:range:)) — Encodes a command that runs a range of commands from an indirect command buffer (ICB).
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes a command that runs an indirect range of commands from an indirect command buffer (ICB).

### Sampling counters
- [sampleCounters(sampleBuffer:sampleIndex:barrier:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/samplecounters(samplebuffer:sampleindex:barrier:)) — Encodes a command that samples hardware counters during the render pass and stores the data into a counter sample buffer.

### Deprecated
- [Deprecated symbols](https://developer.apple.com/documentation/metal/deprecated-symbols) — Review unsupported symbols and their replacements.

## See also

### Encoding a render pass
- [MTL4RenderCommandEncoder](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTLTriangleFillMode](https://developer.apple.com/documentation/metal/mtltrianglefillmode) — Specifies how to rasterize triangle and triangle strip primitives.
- [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) — The vertex winding rule that determines a front-facing primitive.
- [MTLCullMode](https://developer.apple.com/documentation/metal/mtlcullmode) — The mode that determines whether to perform culling and which type of primitive to cull.
- [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) — The geometric primitive type for drawing commands.
- [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) — The index type for an index buffer that references vertices of geometric primitives.
- [MTLDepthClipMode](https://developer.apple.com/documentation/metal/mtldepthclipmode) — The mode that determines how to deal with fragments outside of the near or far planes.
- [MTLVisibilityResultMode](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode) — The mode that determines what, if anything, the GPU writes to the results buffer, after the GPU executes the render pass.
- [MTLVisibilityResultType](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype) — This enumeration controls if Metal accumulates visibility results between render encoders or resets them.
