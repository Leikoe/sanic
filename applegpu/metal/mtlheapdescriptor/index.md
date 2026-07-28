# MTLHeapDescriptor

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor>

A configuration that customizes the behavior for a Metal memory heap.

## Declaration

```swift
class MTLHeapDescriptor
```

## Overview

Create an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) by configuring an [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) instance’s properties and passing it to the [makeHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice).

Each new heap inherits the descriptor’s configuration as you create it, which means you can modify and reuse a descriptor to create other heaps.

## Topics

### Configuring a heap
- [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) — The memory placement strategy for any resources you allocate from the heaps you create with this descriptor.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/storagemode) — The storage mode for the heaps you create with this descriptor.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/cpucachemode) — The CPU cache behavior for any resources you allocate from the heaps you create with this descriptor.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) — The hazard tracking behavior for any resources you allocate from the heaps you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheapdescriptor/resourceoptions) — The combined behavior for any resources you allocate from the heaps you create with this descriptor.
- [size](https://developer.apple.com/documentation/metal/mtlheapdescriptor/size) — The total amount of memory, in bytes, for the heaps you create with this descriptor.
- [sparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/sparsepagesize) — The page size for any resources you allocate from the heaps you create with this descriptor.

### Instance Properties
- [maxCompatiblePlacementSparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/maxcompatibleplacementsparsepagesize) — Specifies the largest sparse page size that the Metal heap supports.

## See also

### Resource memory allocation and management
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) — A memory pool from which you can suballocate resources.
- [MTLHeapType](https://developer.apple.com/documentation/metal/mtlheaptype) — The options you use to choose the heap type.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.
