# useHeaps(_:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheaps(_:)>

Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.

## Declaration

```swift
func useHeaps(_ heaps: [any MTLHeap])
```

## Parameters

- **heaps** — An array of [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances with resources that subsequent draw commands depend on.

## Discussion

You can make the resources in `heaps` *resident* (available in GPU memory) for the remaining duration of the render pass by calling this method. Call the method before encoding draw calls that may access resources within `heaps` through an argument buffer. The method ensures each resource is in a format that’s compatible with the shaders that depend on it.

The method’s applies the [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read) resource usage option to all of the resources within `heaps`, except for textures. The method ignores any texture that has [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget), [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite), or both in its [usage](https://developer.apple.com/documentation/metal/mtltexture/usage) property. For all other textures in `heaps`, the method optimizes each texture’s memory layout for rendering with a sampler. However, your shaders can’t read from those textures by calling this method because the texture needs a different memory layout that’s suitable for reading.

> **Important:**
>  You can instruct Metal to allow a shader to read from texture or write to other resources in heap, by calling [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)).

Methods that apply a usage option for resources (see [Argument buffer resource preparation commands](https://developer.apple.com/documentation/metal/argument-buffer-resource-preparation-commands)) override any previous calls that apply to a resource. For example, you can change the usage option for a buffer to [write](https://developer.apple.com/documentation/metal/mtlresourceusage/write) by passing it to [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)) after calling this method. However, you can’t reverse the call order because this method resets the usage for all resources within `heaps` to [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read), overriding previous calls to [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)).

The method instructs Metal to apply hazard tracking for resources you allocate from a heap that you create with [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked). However, for untracked resources — which come from heaps you create with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) — you need to account for hazards by applying [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) or [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) instances.

> **Note:**
>  The [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) property of a new [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) instance is [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default), which is equivalent to [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) because heaps don’t track resources by default.

Apps typically call the method for heaps that have resources in argument buffers for a *bindless* implementation. For more information about argument buffers and bindless implementations, see [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) and [Go bindless with Metal 3](https://developer.apple.com/videos/play/wwdc2022/10101/), respectively.

## See also

### Deprecated methods
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.
- [use(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
- [use(_:count:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:count:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [use(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from a heap.
- [use(_:count:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/use(_:count:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to the resources you allocate from multiple heaps.
- [textureBarrier()](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/texturebarrier()) — Adds a barrier, which forces any texture read operations to wait until write operations to the same texture finish.
