# removeAllocations(_:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocations(_:)>

Stages multiple resources to leave the residency set’s list of allocations.

## Declaration

```swift
func removeAllocations(_ allocations: [any MTLAllocation])
```

## Parameters

- **allocations** — An array of resource allocations, whose elements can be an arbitrarily mix of [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

## Discussion

Finalize the removal of these resource allocations, and all other changes you stage, by calling a residency set’s [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method.

## See also

### Removing allocations
- [removeAllAllocations()](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallallocations()) — Stages all the resources in the residency set to leave its list of allocations.
- [removeAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocation(_:)) — Stages a single resource to leave the residency set’s list of allocations.
