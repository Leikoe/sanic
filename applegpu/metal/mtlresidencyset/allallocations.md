# allAllocations

*Instance Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencyset/allallocations>

The residency set’s current list of resource allocations.

## Declaration

```swift
var allAllocations: [any MTLAllocation] { get }
```

## Discussion

This property is an array of resource allocations, and its elements can be an arbitrary mix of [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instances.

The residency set updates the property’s value when you call the [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method.

## See also

### Inspecting a residency set
- [label](https://developer.apple.com/documentation/metal/mtlresidencyset/label) — An optional name that can help you identify the residency set.
- [device](https://developer.apple.com/documentation/metal/mtlresidencyset/device) — The Metal device that owns the residency set.
- [containsAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/containsallocation(_:)) — Returns a Boolean value that indicates whether the residency set contains a specific resource allocation.
- [allocationCount](https://developer.apple.com/documentation/metal/mtlresidencyset/allocationcount) — The number of resource allocations in the residency set.
- [allocatedSize](https://developer.apple.com/documentation/metal/mtlresidencyset/allocatedsize) — The amount of resident memory, in bytes, the residency set’s resource allocations consume.
