# Tracking the resource residency of argument buffers

*Article*

<https://developer.apple.com/documentation/metal/tracking-the-resource-residency-of-argument-buffers>

Optimize resource performance within an argument buffer.

## Overview

The Metal driver can’t automatically track the residency of argument buffer resources, but you can track it manually.

### Track argument buffer resource residency manually

Call an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) or [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) method:

- For individual resources, call [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)) or [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)).

- For all resources in a heap, call [useHeap(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:stages:)) or [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)).

These methods perform two important functions:

- They add argument buffer resources to the set of resources that the render or compute pass needs resident.

- They ensure that argument buffer resources are in a format that’s compatible with the required function operation, as an [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) value specifies.

The methods with a `stages` parameter also insert dependency hazards, similar to [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instances for that stage.

Call these methods before issuing any draw or dispatch calls that may access the specified resources.

> **Note:**
>  To track resource access and dependency hazards, use [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instances.
> 
> If all the required resources aren’t resident when executing a render or compute pass, the associated [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance fails.

## See also

### Argument buffers
- [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) — Optimize your app’s performance by grouping your resources into argument buffers.
- [Managing groups of resources with argument buffers](https://developer.apple.com/documentation/metal/managing-groups-of-resources-with-argument-buffers) — Create argument buffers to organize related resources.
- [Indexing argument buffers](https://developer.apple.com/documentation/metal/indexing-argument-buffers) — Assign resource indices within an argument buffer.
- [Rendering terrain dynamically with argument buffers](https://developer.apple.com/documentation/metal/rendering-terrain-dynamically-with-argument-buffers) — Use argument buffers to render terrain in real time with a GPU-driven pipeline.
- [Encoding argument buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-argument-buffers-on-the-gpu) — Use a compute pass to encode an argument buffer and access its arguments in a subsequent render pass.
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) — A representation of an argument within an argument buffer.
- [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) — An interface you can use to encode argument data into an argument buffer.
- [MTLAttributeStrideStatic](https://developer.apple.com/documentation/metal/mtlattributestridestatic)
