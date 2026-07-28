# useResources(_:usage:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 11.0, macOS 10.13, tvOS 11.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresources(_:usage:)>

Ensures kernel calls that the system encodes in subsequent commands have access to multiple resources.

## Declaration

```swift
func useResources(_ resources: [any MTLResource], usage: MTLResourceUsage)
```

## Parameters

- **resources** — A list of [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instances used in one or more argument buffers.
- **usage** — All the applicable access types for use of these resources, including [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read) and [write](https://developer.apple.com/documentation/metal/mtlresourceusage/write). Your resource usage type applies to all resources passed to this method call. For applicable resources, you may be able to prevent the GPU from unnecessarily decompressing color attachments on some devices by setting `usage` to [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read).

## Discussion

You can make many resources *resident* (available in GPU memory) for the remaining duration of the compute pass by calling this method. Call the method before encoding function calls that may access these `resources` through an argument buffer. The method ensures the resource is in a format that’s compatible with the kernels that depend on it.

> **Note:**
>  You don’t need to call this method if you bind a resource for compute kernels to access.

The method also informs Metal when to apply hazard tracking for a resource you create with [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked). For a resource you create with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked), you need to apply an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) or an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) to account for potential reading and writing hazards.

You can reconfigure an individual resource’s `usage` options for subsequent draw calls with the [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) method.

Apps typically call this method for a resource in an argument buffer as a part of their *bindless* implementation. For more information about argument buffers and bindless implementations, see [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) and [Go bindless with Metal 3](https://developer.apple.com/videos/play/wwdc2022/10101/), respectively.

## See also

### Making indirect resources resident
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) — Ensures kernel calls that the system encodes in subsequent commands have access to a resource.
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to all of the resources you allocate from a heap.
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheaps(_:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to all of the resources you allocate from multiple heaps.
