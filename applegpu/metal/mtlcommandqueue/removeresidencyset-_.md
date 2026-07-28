# removeResidencySet(_:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlcommandqueue/removeresidencyset(_:)>

Removes a residency set from a command queue’s list, which means Metal doesn’t apply it to the queue’s command buffers as you commit them.

## Declaration

```swift
func removeResidencySet(_ residencySet: any MTLResidencySet)
```

## Parameters

- **residencySet** — A residency set that contains resource allocations, such as [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

## Discussion

The method doesn’t remove the residency set from command buffers the queue owns with a [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) property that’s equal to [MTLCommandBufferStatus.committed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/committed) or [MTLCommandBufferStatus.scheduled](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled).

See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) and [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) for more information.

## See also

### Detaching residency sets
- [removeResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/removeresidencysets(_:)) — Removes multiple residency sets from a command queue’s list, which means Metal doesn’t apply them to the queue’s command buffers as you commit them.
