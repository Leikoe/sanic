# removeResidencySet(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/removeresidencyset(_:)>

Removes a residency set from a command queue’s list, which means Metal doesn’t apply it to the queue’s command buffers as you commit them.

## Declaration

```swift
func removeResidencySet(_ residencySet: any MTLResidencySet)
```

## Parameters

- **residencySet** — A residency set that contains resource allocations, such as [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

## Discussion

The method doesn’t remove the residency set from command buffers the queue owns with an [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) property that’s equal to [MTLCommandBufferStatus.committed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/committed) or [MTLCommandBufferStatus.scheduled](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled).

See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) and [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) for more information.
