# useHeaps(_:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheaps(_:)>

Ensures the shaders in the render pass’s subsequent draw commands have access to all of the resources you allocate from multiple heaps.

## Declaration

```swift
func useHeaps(_ heaps: [any MTLHeap])
```

## Parameters

- **heaps** — A list of [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances, each of which contain resources used in an argument buffer.

## Discussion

You can make the resources in each of the `heaps` *resident* (available in GPU memory) for the remaining duration of the render pass by calling this method. Call the method before encoding draw calls that may access resources within the `heaps` through an argument buffer. The method ensures each resource is in a format that’s compatible with the kernels that depend on it.

This method applies the [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read) resource usage option to all of the resources within `heap`, except for textures. The method ignores any texture that has [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget), [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite), or both in its [usage](https://developer.apple.com/documentation/metal/mtltexture/usage) property. For all other textures in `heap`, the method optimizes each texture’s memory layout for rendering with a sampler. However, your kernels can’t read from those textures by calling this method because the texture needs a different memory layout that’s suitable for reading.

> **Important:**
>  You can instruct Metal to allow a kernel to read from a texture or write to resources in the heap by calling [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:))

Methods that apply a usage option for resources (see Encoding Resident Resources) override any previous calls that apply to a resource. For example, you can change the usage option for a buffer allocated in `heap` to [write](https://developer.apple.com/documentation/metal/mtlresourceusage/write) by passing it to [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresources(_:usage:)) after calling this method. However, you can’t reverse the call order because this method resets the usage for all resources within `heap` to [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read), overriding previous calls to [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)).

This method instructs Metal to apply hazard tracking for resources you allocate from a heap that you create with [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked). However, for untracked resources — which come from heaps you create with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) — you need to account for hazards by applying [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) or [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) instances.

> **Note:**
>  The [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) property of a new [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) instance is [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default), which is equivalent to [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) because heaps don’t track resources by default.

Apps typically call the method for heaps that have resources in argument buffers for a *bindless* implementation. For more information about argument buffers and bindless implementations, see [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) and [Go bindless with Metal 3](https://developer.apple.com/videos/play/wwdc2022/10101/), respectively.

## See also

### Making indirect resources resident
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) — Ensures kernel calls that the system encodes in subsequent commands have access to a resource.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresources(_:usage:)) — Ensures kernel calls that the system encodes in subsequent commands have access to multiple resources.
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to all of the resources you allocate from a heap.
