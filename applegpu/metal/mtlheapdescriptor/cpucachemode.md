# cpuCacheMode

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor/cpucachemode>

The CPU cache behavior for any resources you allocate from the heaps you create with this descriptor.

## Declaration

```swift
var cpuCacheMode: MTLCPUCacheMode { get set }
```

## Discussion

This property’s default value is [MTLCPUCacheMode.defaultCache](https://developer.apple.com/documentation/metal/mtlcpucachemode/defaultcache).

The resources you allocate from a heap inherit that heap’s CPU cache mode.

## See also

### Configuring a heap
- [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) — The memory placement strategy for any resources you allocate from the heaps you create with this descriptor.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/storagemode) — The storage mode for the heaps you create with this descriptor.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) — The hazard tracking behavior for any resources you allocate from the heaps you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheapdescriptor/resourceoptions) — The combined behavior for any resources you allocate from the heaps you create with this descriptor.
- [size](https://developer.apple.com/documentation/metal/mtlheapdescriptor/size) — The total amount of memory, in bytes, for the heaps you create with this descriptor.
- [sparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/sparsepagesize) — The page size for any resources you allocate from the heaps you create with this descriptor.
