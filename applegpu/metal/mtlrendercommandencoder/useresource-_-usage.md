# useResource(_:usage:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:)>

Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.

## Declaration

```swift
func useResource(_ resource: any MTLResource, usage: MTLResourceUsage)
```

## Parameters

- **resource** — An [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instance that subsequent draw commands depend on.
- **usage** — All the applicable access types the render pass’s shaders use for `resource`, including [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read) and [write](https://developer.apple.com/documentation/metal/mtlresourceusage/write). For applicable resources, you may be able to prevent the GPU from unnecessarily decompressing color attachments on some devices by setting `usage` to [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read).

## Discussion

You can make a resource *resident* (available in GPU memory) for the remaining duration of the render pass by calling this method. Call the method before encoding draw calls that may access `resource` through an argument buffer. The method ensures the resource is in a format that’s compatible with the shaders that depend on it.

> **Note:**
>  You don’t need to call this method if you bind a resource to a shader stage.

For example, you can explicitly bind resources for the vertex stage with the methods in the [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) collection.

The method also informs Metal when to apply hazard tracking for a resource you create with [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked). For a resource you create with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked), you need to apply an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) or an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) to account for potential reading and writing hazards.

You can reconfigure an individual resource’s `usage` options for subsequent draw calls in the same render pass by calling this method again.

Apps typically call the method for a resource in an argument buffer as a part of their *bindless* implementation. For more information about argument buffers and bindless implementations, see [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) and [Go bindless with Metal 3](https://developer.apple.com/videos/play/wwdc2022/10101/), respectively.

## See also

### Deprecated methods
- [use(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
- [use(_:count:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:count:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [use(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheaps(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.
- [use(_:count:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:count:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.
- [textureBarrier()](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/texturebarrier()) — Adds a barrier, which forces any texture read operations to wait until write operations to the same texture finish.
