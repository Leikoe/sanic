# Argument buffer resource preparation commands

*API Collection*

<https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands>

Load individual resources and multiple resources within a heap into GPU memory so that they’re available to shaders through argument buffers.

## Overview

These methods encode commands that load resources into GPU memory, making them accessible to your shaders through argument buffers. To load an individual resource, call the [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)) method, or another resource-based method. Alternatively, you can load all the resources within a heap by calling the [useHeap(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:stages:)) method or another heap-based method.

> **Important:**
>  The heap-based methods don’t provide a `usage` parameter (see [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage)) and set the usage for the resources within each heap to [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read).

To give shaders write or read/write access to specific resources within a heap, call a resource-based method after the heap-based method. Metal combines usage modes you set for a resource through both heap and resource methods.

For more information, see [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers).

## Topics

### Loading individual resources for argument buffers
- [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.
- [useResources(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.

### Loading heaps and the resources they contain for argument buffers
- [useHeap(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [useHeaps(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheaps(_:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.

## See also

### Resource preparation commands
- [Mesh and object shader resource preparation commands](https://developer.apple.com/documentation/metal/mesh-and-object-shader-resource-preparation-commands) — Assign resources to mesh and object shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) — Assign resources to vertex shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Fragment shader resource preparation commands](https://developer.apple.com/documentation/metal/fragment-shader-resource-preparation-commands) — Assign resources to fragment shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
- [Tile shaders resource preparation commands](https://developer.apple.com/documentation/metal/tile-shaders-resource-preparation-commands) — Assign resources to tile shaders, including buffers, textures, acceleration structures, sampler states, and function tables.
