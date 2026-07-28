# hazardTrackingMode

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode>

The hazard tracking behavior for any resources you allocate from the heaps you create with this descriptor.

## Declaration

```swift
var hazardTrackingMode: MTLHazardTrackingMode { get set }
```

## Discussion

This property’s default value is [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default), which is equivalent to [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) for a heap.

The resources you allocate from a heap inherit that heap’s hazard tracking mode.

## See also

### Configuring a heap
- [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) — The memory placement strategy for any resources you allocate from the heaps you create with this descriptor.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/storagemode) — The storage mode for the heaps you create with this descriptor.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/cpucachemode) — The CPU cache behavior for any resources you allocate from the heaps you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheapdescriptor/resourceoptions) — The combined behavior for any resources you allocate from the heaps you create with this descriptor.
- [size](https://developer.apple.com/documentation/metal/mtlheapdescriptor/size) — The total amount of memory, in bytes, for the heaps you create with this descriptor.
- [sparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/sparsepagesize) — The page size for any resources you allocate from the heaps you create with this descriptor.
