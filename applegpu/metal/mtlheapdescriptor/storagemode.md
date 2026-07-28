# storageMode

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor/storagemode>

The storage mode for the heaps you create with this descriptor.

## Declaration

```swift
var storageMode: MTLStorageMode { get set }
```

## Discussion

For devices with Apple silicon, you can create a heap with either the [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) or the [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) storage mode. However, you can only create heaps with private storage on macOS devices without Apple silicon.

The resources you allocate from a heap inherit that heap’s storage mode. This property’s default value is [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private).

## See also

### Configuring a heap
- [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) — The memory placement strategy for any resources you allocate from the heaps you create with this descriptor.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/cpucachemode) — The CPU cache behavior for any resources you allocate from the heaps you create with this descriptor.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) — The hazard tracking behavior for any resources you allocate from the heaps you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheapdescriptor/resourceoptions) — The combined behavior for any resources you allocate from the heaps you create with this descriptor.
- [size](https://developer.apple.com/documentation/metal/mtlheapdescriptor/size) — The total amount of memory, in bytes, for the heaps you create with this descriptor.
- [sparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/sparsepagesize) — The page size for any resources you allocate from the heaps you create with this descriptor.
