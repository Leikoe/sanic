# useResources(_:usage:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useresources(_:usage:)>

Makes multiple resources available to the acceleration structure pass.

## Declaration

```swift
func useResources(_ resources: [any MTLResource], usage: MTLResourceUsage)
```

## Parameters

- **resources** — An array of resources within an argument buffer.
- **usage** — Options that indicate how a GPU function accesses each resource in `resources`.

## Discussion

This method makes the resources resident for the duration of a compute pass and ensures that they are in a format compatible with the compute function.

Call this method before issuing any dispatch calls that may access the resource. Calling this method again, or calling [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)), overwrites any previously specified usage options for future dispatch calls within the same compute command encoder.

> **Note:**
>  You can track resource access and dependency hazards with [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) instances.

## See also

### Making indirect resources resident
- [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useheap(_:)) — Makes the resources contained in the specified heap available to the acceleration structure pass.
- [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useheaps(_:)) — Makes the resources contained in the specified heaps available to the acceleration structure pass.
- [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecommandencoder/useresource(_:usage:)) — Makes a resource available to the acceleration structure pass.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.
