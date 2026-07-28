# addAllocations(_:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlresidencyset/addallocations(_:)>

Stages multiple resources to join the residency set’s list of allocations.

## Declaration

```swift
func addAllocations(_ allocations: [any MTLAllocation])
```

## Parameters

- **allocations** — An array of resource allocations, whose elements can be an arbitrarily mix of [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

## Discussion

Finalize the inclusion of these resource allocations, and all other changes you stage, by calling a residency set’s [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method.

## See also

### Adding allocations
- [addAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)) — Stages a single resource to join the residency set’s list of allocations.
