# addResidencySets(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/addresidencysets(_:)>

Applies multiple residency sets to a queue, which Metal applies to the queue’s command buffers as you commit them.

## Declaration

```swift
func addResidencySets(_ residencySets: [any MTLResidencySet])
```

## Parameters

- **residencySets** — An array of residency sets, each of which contains resource allocations, such as [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

## Discussion

Each command queue can maintain a list of up to 32 different residency sets. See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) and [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) for more information.
