# useResource(_:usage:stages:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)>

Ensures the shaders in the render pass’s subsequent draw commands have access to a resource.

## Declaration

```swift
func useResource(_ resource: any MTLResource, usage: MTLResourceUsage, stages: MTLRenderStages)
```

## Parameters

- **resource** — An [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) instance that subsequent draw commands depend on.
- **usage** — All the applicable access types the render pass’s shaders use for the resource, including [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read) and [write](https://developer.apple.com/documentation/metal/mtlresourceusage/write). For applicable resources, you may be able to prevent the GPU from unnecessarily decompressing color attachments on some devices by setting `usage` to [read](https://developer.apple.com/documentation/metal/mtlresourceusage/read).
- **stages** — All the render stages that depend on `resource`, including [object](https://developer.apple.com/documentation/metal/mtlrenderstages/object), [mesh](https://developer.apple.com/documentation/metal/mtlrenderstages/mesh), [vertex](https://developer.apple.com/documentation/metal/mtlrenderstages/vertex), [fragment](https://developer.apple.com/documentation/metal/mtlrenderstages/fragment), and [tile](https://developer.apple.com/documentation/metal/mtlrenderstages/tile).

## Discussion

You can make a resource *resident* (available in GPU memory) for the remaining duration of the render pass by calling this method. Call the method before encoding draw calls that may access `resource` through an argument buffer. The method ensures the resource is in a format that’s compatible with the shaders that depend on it.

> **Note:**
>  You don’t need to call this method if you bind a resource to a shader stage.

For example, you can explicitly bind resources for the vertex stage with the methods in the [Vertex shader resource preparation commands](https://developer.apple.com/documentation/metal/vertex-shader-resource-preparation-commands) collection.

The method also informs Metal when to apply hazard tracking for a resource you create with [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked). For a resource you create with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked), you need to apply an [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) or an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) to account for potential reading and writing hazards.

You can reconfigure an individual resource’s `usage` options for subsequent draw calls in the same render pass by calling this method again.

Apps typically call the method for a resource in an argument buffer as a part of their *bindless* implementation. For more information about argument buffers and bindless implementations, see [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) and [Go bindless with Metal 3](https://developer.apple.com/videos/play/wwdc2022/10101/), respectively.

## See also

### Loading individual resources for argument buffers
- [useResources(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:stages:)) — Ensures the shaders in the render pass’s subsequent draw commands have access to multiple resources.
