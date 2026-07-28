# MTL4RenderCommandEncoder

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder>

Encodes configuration and draw commands for a single render pass into a command buffer.

## Declaration

```swift
protocol MTL4RenderCommandEncoder : MTL4CommandEncoder
```

## Overview

A render pass draws a scene, or a component within a scene, to its render *attachments*, the outputs of a render pass. You can render to those outputs with various approaches, including techniques that apply the following:

- Primitive drawing

- Mesh drawing

- Ray tracing

- Dispatching tile shaders

Create a render encoder by calling a factory method of an [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) instance, such as [makeRenderCommandEncoder(descriptor:options:)](https://developer.apple.com/documentation/metal/mtl4commandbuffer/makerendercommandencoder(descriptor:options:)).

To configure the render pass for your first drawing commands, start with a pipeline state by passing an [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) instance to the encoder’s [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setrenderpipelinestate(_:)) method. You create the pipeline states your render pass needs, typically ahead of time, by calling one or more [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods (see [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation)).

> **Tip:**
>  Avoid visual stutter by creating pipeline states at a noncritical time, such as during launch, because of the time it can take to make them.

Configure other encoder settings by calling the methods in the configuration groups below, such as [setViewport(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewport(_:)) for the viewport, [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrect(_:)) for the scissor rectangle, and [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthstencilstate(_:)) for depth and stencil tests.

Bind resources by calling [setArgumentTable(_:stages:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setargumenttable(_:stages:)) with an [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) instance. This table contains the buffers, textures, and other resources your shaders depend on.

Encode drawing commands after you configure the state and resources the commands depend on. The encoder maintains its current state and applies it to all subsequent draw commands. For drawing commands that need different states or resources, reconfigure the render pass appropriately and then encode those draw commands. Repeat the process for each batch of drawing commands that depend on the same render pass configuration and resources.

When you finish encoding the render pass’s commands, finalize it into the command buffer by calling the encoder’s [endEncoding()](https://developer.apple.com/documentation/metal/mtl4commandencoder/endencoding()) method.

### Command stages

Most render commands apply to one or more stages within a pass. The following table shows which stages apply to each command:

| Function | MTLStages |
|---|---|
| [drawPrimitives(primitiveType:vertexStart:vertexCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:baseinstance:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawPrimitives(primitiveType:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:indirectbuffer:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:basevertex:baseinstance:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawIndexedPrimitives(primitiveType:indexType:indexBuffer:indexBufferLength:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indextype:indexbuffer:indexbufferlength:indirectbuffer:)) | [vertex](https://developer.apple.com/documentation/metal/mtlstages/vertex)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawMeshThreads(threadsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreads(threadspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) | [object](https://developer.apple.com/documentation/metal/mtlstages/object)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawMeshThreadgroups(threadgroupsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(threadgroupspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) | [object](https://developer.apple.com/documentation/metal/mtlstages/object)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [drawMeshThreadgroups(indirectBuffer:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(indirectbuffer:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) | [object](https://developer.apple.com/documentation/metal/mtlstages/object)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[mesh](https://developer.apple.com/documentation/metal/mtlstages/mesh)![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) |
| [dispatchThreadsPerTile(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/dispatchthreadspertile(_:)) | [tile](https://developer.apple.com/documentation/metal/mtlstages/tile) |
| [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:range:))![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png)[executeCommandsInBuffer:withRange:](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommandsinbuffer:withrange:) | None |
| [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:indirectbuffer:)) | None |
| [writeTimestamp(granularity:after:counterHeap:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/writetimestamp(granularity:after:counterheap:index:)) | None |

Draw commands don’t apply to [fragment](https://developer.apple.com/documentation/metal/mtlstages/fragment) when the [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) for the draw disables rasterization. See [isRasterizationEnabled](https://developer.apple.com/documentation/metal/mtl4renderpipelinedescriptor/israsterizationenabled).

Mesh draw commands don’t apply to [object](https://developer.apple.com/documentation/metal/mtlstages/object) when the [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) for the draw doesn’t have an object shader.

The [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:range:)) and [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:indirectbuffer:)) commands don’t apply to any stage, which means you can’t use a barrier to wait for all commands in an indirect command buffer to complete. However, each command within the [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) applies to the same stages as when you encode the equivalent command directly.

For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Configuring pipeline state
- [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setrenderpipelinestate(_:)) — Configures this encoder with a render pipeline state that applies to your subsequent draw commands.

### Configuring the actions for attachments
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcolorstoreaction(_:index:)) — Configures the store action for a color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthstoreaction(_:)) — Configures the store action for the depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilstoreaction(_:)) — Configures the store action for the stencil attachment.

### Configuring blend behavior
- [setBlendColor(red:green:blue:alpha:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setblendcolor(red:green:blue:alpha:)) — Configures each pixel component value, including alpha, for the render pipeline’s constant blend color.
- [setColorAttachmentMap(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcolorattachmentmap(_:)) — Sets the mapping from logical shader color output to physical render pass color attachments.

### Configuring rendering behavior
- [setTriangleFillMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/settrianglefillmode(_:)) — Configures how subsequent draw commands rasterize triangle and triangle strip primitives.
- [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setfrontfacing(_:)) — Configures the vertex winding order that determines which face of a geometric primitive is the front one.
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setcullmode(_:)) — Controls whether Metal culls front facing primitives, back facing primitives, or culls no primitives at all.

### Configuring depth and stencil behavior
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthstencilstate(_:)) — Configures this encoder with a depth stencil state that applies to your subsequent draw commands.
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthbias(_:slopescale:clamp:)) — Configures the adjustments a render pass applies to depth values from fragment shader functions by a scaling factor and bias.
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthclipmode(_:)) — Controls the behavior for fragments outside of the near or far planes.
- [setDepthTestBounds(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setdepthtestbounds(_:)) — Configures the range for depth bounds testing.
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilreferencevalue(_:)) — Configures this encoder with a reference value for stencil testing.
- [setStencilReferenceValue(front:back:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setstencilreferencevalue(front:back:)) — Configures the encoder with different stencil test reference values for front-facing and back-facing primitives.

### Configuring viewport and scissor behavior
- [setViewport(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewport(_:)) — Sets the viewport which that transforms vertices from normalized device coordinates to window coordinates.
- [setViewports(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setviewports(_:)) — Sets an array of viewports to transform vertices from normalized device coordinates to window coordinates.
- [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrect(_:)) — Sets a scissor rectangle to discard fragments outside a specific area.
- [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setscissorrects(_:)) — Sets an array of scissor rectangles for a fragment scissor test.

### Configuring visibility testing
- [setVisibilityResultMode(_:offset:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setvisibilityresultmode(_:offset:)) — Configures a visibility test for Metal to run, and the destination for any results it generates.

### Configuring vertex amplification
- [setVertexAmplificationCount(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setvertexamplificationcount(_:)-85tu1) — Sets the vertex amplification count and its view mapping for each amplification ID.
- [setVertexAmplificationCount(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setvertexamplificationcount(_:)-911ja) — Sets the vertex amplification count and its view mapping for each amplification ID.

### Configuring persistent threadgroup memory
- [setObjectThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setobjectthreadgroupmemorylength(_:index:)) — Configures the size of a threadgroup memory buffer for a threadgroup argument in the object shader function.
- [setThreadgroupMemoryLength(_:offset:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setthreadgroupmemorylength(_:offset:index:)) — Configures the size of a threadgroup memory buffer for a threadgroup argument in the fragment and tile shader functions.

### Binding argument tables
- [setArgumentTable(_:stages:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/setargumenttable(_:stages:)) — Associates an argument table with a set of render stages.

### Drawing with vertices
- [drawPrimitives(primitiveType:vertexStart:vertexCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:)) — Encodes a draw command that renders an instance of a geometric primitive.
- [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive.
- [drawPrimitives(primitiveType:vertexStart:vertexCount:instanceCount:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:vertexstart:vertexcount:instancecount:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive, starting with a custom instance identification number.
- [drawPrimitives(primitiveType:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawprimitives(primitivetype:indirectbuffer:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indirect arguments.

### Drawing with indexed vertices
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:)) — Encodes a draw command that renders an instance of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices.
- [drawIndexedPrimitives(primitiveType:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:baseVertex:baseInstance:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indexcount:indextype:indexbuffer:indexbufferlength:instancecount:basevertex:baseinstance:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices, starting with a custom vertex and instance.
- [drawIndexedPrimitives(primitiveType:indexType:indexBuffer:indexBufferLength:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawindexedprimitives(primitivetype:indextype:indexbuffer:indexbufferlength:indirectbuffer:)) — Encodes a draw command that renders multiple instances of a geometric primitive with indexed vertices and indirect arguments.

### Drawing with meshes
- [drawMeshThreads(threadsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreads(threadspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threads.
- [drawMeshThreadgroups(threadgroupsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(threadgroupspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threadgroups.
- [drawMeshThreadgroups(indirectBuffer:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(indirectbuffer:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with indirect arguments.

### Drawing with tile shaders
- [dispatchThreadsPerTile(_:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/dispatchthreadspertile(_:)) — Encodes a command that invokes a tile shader function from the encoder’s current tile render pipeline state.
- [tileWidth](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/tilewidth) — Sets the width of a tile for this render pass.
- [tileHeight](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/tileheight) — Sets the height of a tile for this render pass.

### Running commands from indirect command buffers
- [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:range:)) — Encodes a command that runs a range of commands from an indirect command buffer.
- [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:indirectbuffer:)) — Encodes a command that runs an indirect range of commands from an indirect command buffer.

### Sampling counters
- [writeTimestamp(granularity:after:counterHeap:index:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/writetimestamp(granularity:after:counterheap:index:)) — Writes a GPU timestamp into the given [MTL4CounterHeap](https://developer.apple.com/documentation/metal/mtl4counterheap) at `index` after `stage` completes.

## See also

### Encoding a render pass
- [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) — Encodes configuration and draw commands for a single render pass into a command buffer.
- [MTL4RenderEncoderOptions](https://developer.apple.com/documentation/metal/mtl4renderencoderoptions) — Custom render pass options you specify at encoder creation time.
- [MTLTriangleFillMode](https://developer.apple.com/documentation/metal/mtltrianglefillmode) — Specifies how to rasterize triangle and triangle strip primitives.
- [MTLWinding](https://developer.apple.com/documentation/metal/mtlwinding) — The vertex winding rule that determines a front-facing primitive.
- [MTLCullMode](https://developer.apple.com/documentation/metal/mtlcullmode) — The mode that determines whether to perform culling and which type of primitive to cull.
- [MTLPrimitiveType](https://developer.apple.com/documentation/metal/mtlprimitivetype) — The geometric primitive type for drawing commands.
- [MTLIndexType](https://developer.apple.com/documentation/metal/mtlindextype) — The index type for an index buffer that references vertices of geometric primitives.
- [MTLDepthClipMode](https://developer.apple.com/documentation/metal/mtldepthclipmode) — The mode that determines how to deal with fragments outside of the near or far planes.
- [MTLVisibilityResultMode](https://developer.apple.com/documentation/metal/mtlvisibilityresultmode) — The mode that determines what, if anything, the GPU writes to the results buffer, after the GPU executes the render pass.
- [MTLVisibilityResultType](https://developer.apple.com/documentation/metal/mtlvisibilityresulttype) — This enumeration controls if Metal accumulates visibility results between render encoders or resets them.
