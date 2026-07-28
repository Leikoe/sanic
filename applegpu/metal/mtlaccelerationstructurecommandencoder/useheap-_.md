# useHeap(_:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useheap(_:)>

Makes the resources contained in the specified heap available to the acceleration structure pass.

## Declaration

```swift
func useHeap(_ heap: any MTLHeap)
```

## Parameters

- **heap** — A heap that contains resources within an argument buffer.

## Discussion

This method makes all the resources in the heap resident for the duration of a compute pass and ensures that they’re in a format compatible with the compute function.

Call this method before issuing any dispatch calls that may access the resources in the heap.

You can only read or sample resources in the specified heap. This method ignores render targets (textures that specify a [renderTarget](https://developer.apple.com/documentation/metal/mtltextureusage/rendertarget) usage option) and writable textures (textures that specify a [shaderWrite](https://developer.apple.com/documentation/metal/mtltextureusage/shaderwrite) usage option) within the heap. To use these resources, you need to call the [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) method instead.

> **Note:**
>  You can synchronize memory operations to address dependency hazards with [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instances.

## See also

### Making indirect resources resident
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useheaps(_:)) — Makes the resources contained in the specified heaps available to the acceleration structure pass.
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useresource(_:usage:)) — Makes a resource available to the acceleration structure pass.
- [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useresources(_:usage:)) — Makes multiple resources available to the acceleration structure pass.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.
