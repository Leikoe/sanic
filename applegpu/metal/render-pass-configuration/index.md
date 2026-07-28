# Render pass configuration

*API Collection*

<https://developer.apple.com/documentation/metal/render-pass-configuration>

Set a render pass’s pipeline state, attachment actions, viewports, and so on, that affect subsequent drawing commands.

## Overview

These methods encode commands that configure the render pass for all subsequent drawing commands. The most important configuration is the pipeline state (see [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate)), which you configure by calling the [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) method.

## Topics

### Configuring pipeline state
- [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) — Configures the encoder with a render or tile pipeline state that applies to your subsequent draw commands.

### Configuring the actions for attachments
- [setColorStoreAction(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreaction(_:index:)) — Configures the store action for a color attachment.
- [setColorStoreActionOptions(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorstoreactionoptions(_:index:)) — Configures the store action options for a color attachment.
- [setDepthStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreaction(_:)) — Configures the store action for the depth attachment.
- [setDepthStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstoreactionoptions(_:)) — Configures the store action options for the depth attachment.
- [setStencilStoreAction(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreaction(_:)) — Configures the store action for the stencil attachment.
- [setStencilStoreActionOptions(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilstoreactionoptions(_:)) — Configures the store action options for the stencil attachment.

### Configuring blend behavior
- [setBlendColor(red:green:blue:alpha:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setblendcolor(red:green:blue:alpha:)) — Configures each pixel component value, including alpha, for the render pipeline’s constant blend color.
- [setColorAttachmentMap(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcolorattachmentmap(_:)) — Sets the mapping from logical shader color output to physical render pass color attachments.

### Configuring rendering behavior
- [setTriangleFillMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settrianglefillmode(_:)) — Configures how subsequent draw commands rasterize triangle and triangle strip primitives.
- [setFrontFacing(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfrontfacing(_:)) — Configures which face of a primitive, such as a triangle, is the front.
- [setCullMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setcullmode(_:)) — Configures how the render pipeline determines which primitives to remove.

### Configuring depth and stencil behavior
- [setDepthStencilState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthstencilstate(_:)) — Configures the combined depth and stencil state.
- [setDepthBias(_:slopeScale:clamp:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthbias(_:slopescale:clamp:)) — Configures the adjustments a render pass applies to depth values from fragment functions by a scaling factor and bias.
- [setDepthClipMode(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthclipmode(_:)) — Configures how the render pipeline handles fragments outside the near and far planes of the view frustum.
- [setDepthTestBounds(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setdepthtestbounds(_:)) — Configures the range for depth bounds testing.
- [setStencilReferenceValue(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalue(_:)) — Configures the same comparison value for front- and back-facing primitives.
- [setStencilReferenceValues(front:back:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setstencilreferencevalues(front:back:)) — Configures different comparison values for front- and back-facing primitives.

### Configuring viewport and scissor behavior
- [setViewport(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewport(_:)) — Configures the render pipeline with a viewport that applies a transformation and a clipping rectangle.
- [setViewports(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setviewports(_:)) — Configures the render pipeline with multiple viewports that apply transformations and clipping rectangles.
- [setScissorRect(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrect(_:)) — Configures a rectangle for the fragment scissor test.
- [setScissorRects(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setscissorrects(_:)) — Configures multiple rectangles for the fragment scissor test.

### Configuring visibility testing
- [setVisibilityResultMode(_:offset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvisibilityresultmode(_:offset:)) — Configures which visibility test the GPU runs and the destination for any results it generates.

### Configuring vertex amplification
- [setVertexAmplificationCount(_:viewMappings:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexamplificationcount(_:viewmappings:)) — Configures the number of output vertices the render pipeline produces for each input vertex, optionally with render target and viewport offsets.

### Configuring tessellation factors
- [setTessellationFactorScale(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorscale(_:)) — Configures the scale factor for per-patch tessellation factors.
- [setTessellationFactorBuffer(_:offset:instanceStride:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settessellationfactorbuffer(_:offset:instancestride:)) — Configures the per-patch tessellation factors for any subsequent patch-drawing commands.

### Configuring persistent threadgroup memory
- [setObjectThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectthreadgroupmemorylength(_:index:)) — Configures the size of a threadgroup memory buffer for an entry in the object argument table.
- [setThreadgroupMemoryLength(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setthreadgroupmemorylength(_:offset:index:)) — Configures the size of a threadgroup memory buffer for an entry in the fragment or tile shader argument table.
