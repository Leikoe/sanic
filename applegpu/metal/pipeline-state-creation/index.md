# Pipeline state creation

*API Collection*

<https://developer.apple.com/documentation/metal/pipeline-state-creation>

Create pipeline states for render and compute passes, samplers, depth and stencil states, and indirect command buffers.

## Overview

Use these methods to create instances of various state types for a render or compute pass (see [Render passes](https://developer.apple.com/documentation/metal/render-passes) and [Compute passes](https://developer.apple.com/documentation/metal/compute-passes), respectively).

You can create multiple [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) instances for a single render pass encoder ([MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder)) that each apply to different types of render commands. For example, a single render pass can render primitives with vertices, then meshes, and finish with a tile shader command, each with a different pipeline. To create these pipelines, configure instances of [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor), [MTLMeshRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor), and [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor). Then pass those descriptors to the [makeRenderPipelineState(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:completionhandler:)), [makeRenderPipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:completionhandler:)-1wvya) and [makeRenderPipelineState(tileDescriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:completionhandler:)) methods (or a counterpart method), respectively.

> **Important:**
>  Only create reflection (see [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection)) instances if you need them, because each one can require a significant amount of memory.

## Topics

### Creating render pipeline states with vertex shaders
- [makeRenderPipelineState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:)) — Synchronously creates a render pipeline state.
- [makeRenderPipelineState(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:completionhandler:)) — Asynchronously creates a render pipeline state.
- [makeRenderPipelineState(descriptor:options:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:)-89vxc) — Synchronously creates a render pipeline state and reflection information in a tuple.
- [makeRenderPipelineState(descriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:reflection:)) — Synchronously creates a render pipeline state and reflection information.
- [makeRenderPipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:completionhandler:)-5gdww) — Asynchronously creates a render pipeline state and reflection information.

### Creating render pipeline states with mesh shaders
- [makeRenderPipelineState(descriptor:options:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:)-yrak) — Synchronously creates a mesh render pipeline state and reflection information in a tuple.
- [makeRenderPipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:completionhandler:)-1wvya) — Asynchronously creates a mesh render pipeline state and reflection information.

### Creating tile render pipeline states
- [makeRenderPipelineState(tileDescriptor:options:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:)) — Synchronously creates a tile shader’s render pipeline state and reflection information in a tuple.
- [makeRenderPipelineState(tileDescriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:reflection:)) — Synchronously creates a tile shader’s render pipeline state and reflection information.
- [makeRenderPipelineState(tileDescriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:completionhandler:)) — Asynchronously creates a tile shader’s render pipeline state and reflection information.

### Creating compute pipeline states
- [makeComputePipelineState(descriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(descriptor:options:reflection:)) — Synchronously creates a compute pipeline state and reflection information.
- [makeComputePipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(descriptor:options:completionhandler:)) — Asynchronously creates a compute pipeline state and reflection information.
- [makeComputePipelineState(function:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:)) — Synchronously creates a compute pipeline state with a function instance.
- [makeComputePipelineState(function:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:completionhandler:)) — Asynchronously creates a compute pipeline state with a function instance.
- [makeComputePipelineState(function:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:reflection:)) — Synchronously creates a compute pipeline state and reflection with a function instance.
- [makeComputePipelineState(function:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:completionhandler:)) — Asynchronously creates a compute pipeline state and reflection with a function instance.

### Creating depth and stencil states
- [makeDepthStencilState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makedepthstencilstate(descriptor:)) — Creates a depth-stencil state instance.

### Supporting types
- [MTLNewRenderPipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewrenderpipelinestatecompletionhandler) — A completion handler signature a method calls when it finishes creating a render pipeline.
- [MTLNewRenderPipelineStateWithReflectionCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewrenderpipelinestatewithreflectioncompletionhandler) — A completion handler signature a method calls when it finishes creating a render pipeline and reflection information.
- [MTLNewComputePipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewcomputepipelinestatecompletionhandler) — A completion handler signature a method calls when it finishes creating a compute pipeline.
- [MTLNewComputePipelineStateWithReflectionCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewcomputepipelinestatewithreflectioncompletionhandler) — A completion handler signature a method calls when it finishes creating a compute pipeline and reflection information.

## See also

### Working with GPU devices
- [Device inspection](https://developer.apple.com/documentation/metal/device-inspection) — Locate and identify a GPU and the features it supports, and sample its counters.
- [Work submission](https://developer.apple.com/documentation/metal/work-submission) — Create queues that submit work to the GPU or load assets into GPU resources, and indirect command buffers that group your frequent commands together.
- [Resource creation](https://developer.apple.com/documentation/metal/resource-creation) — Load assets with input/output queues and make various resource instances, such as buffers, textures, acceleration structures, and memory heaps.
- [Shader library and archive creation](https://developer.apple.com/documentation/metal/shader-library-and-archive-creation) — Create static and dynamic shader libraries, and binary shader archives.
