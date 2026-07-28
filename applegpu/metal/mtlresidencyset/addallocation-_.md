# addAllocation(_:)

*Instance Method · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)>

Stages a single resource to join the residency set’s list of allocations.

## Declaration

```swift
func addAllocation(_ allocation: any MTLAllocation)
```

## Parameters

- **allocation** — A resource allocation, such as an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), or [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap).

## Discussion

Finalize the inclusion of these resource allocations, and all other changes you stage, by calling a residency set’s [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method.

## See also

### Adding allocations
- [addAllocations(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocations(_:)) — Stages multiple resources to join the residency set’s list of allocations.
