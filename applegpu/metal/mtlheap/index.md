# MTLHeap

*Protocol · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap>

A memory pool from which you can suballocate resources.

## Declaration

```swift
protocol MTLHeap : MTLAllocation
```

## Overview

Don’t implement this protocol yourself; instead, to create a heap, configure an [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) instance and call the [makeHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

You suballocate resources from a heap and make them *aliasable* or *non-aliasable*. A sub-allocated resource is non-aliased by default, preventing future resources allocated from the heap from using its memory. Resources are *aliased* when they share the same memory allocation on a heap.

All resources sub-allocated from the same heap share the same storage mode and CPU cache mode. You can make heaps purgeable, but not the resources allocated from the heap; they can only reflect the heap’s purgeability state.

## Topics

### Naming and identifying a heap
- [label](https://developer.apple.com/documentation/metal/mtlheap/label) — A string that identifies the heap.

### Creating buffers from a heap
- [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtlheap/makebuffer(length:options:)) — Creates a buffer on the heap.
- [makeBuffer(length:options:offset:)](https://developer.apple.com/documentation/metal/mtlheap/makebuffer(length:options:offset:)) — Creates a buffer at a specified offset on the heap.

### Creating textures from a heap
- [makeTexture(descriptor:)](https://developer.apple.com/documentation/metal/mtlheap/maketexture(descriptor:)) — Creates a texture on the heap.
- [makeTexture(descriptor:offset:)](https://developer.apple.com/documentation/metal/mtlheap/maketexture(descriptor:offset:)) — Creates a texture at a specified offset on the heap.

### Creating acceleration structure from a heap
- [makeAccelerationStructure(size:)](https://developer.apple.com/documentation/metal/mtlheap/makeaccelerationstructure(size:))
- [makeAccelerationStructure(size:offset:)](https://developer.apple.com/documentation/metal/mtlheap/makeaccelerationstructure(size:offset:))
- [makeAccelerationStructure(descriptor:)](https://developer.apple.com/documentation/metal/mtlheap/makeaccelerationstructure(descriptor:))
- [makeAccelerationStructure(descriptor:offset:)](https://developer.apple.com/documentation/metal/mtlheap/makeaccelerationstructure(descriptor:offset:))

### Configuring a heap’s purgeable state
- [setPurgeableState(_:)](https://developer.apple.com/documentation/metal/mtlheap/setpurgeablestate(_:)) — Sets the purgeable state of the heap.

### Checking a heap’s size information
- [maxAvailableSize(alignment:)](https://developer.apple.com/documentation/metal/mtlheap/maxavailablesize(alignment:)) — The maximum size of a resource, in bytes, that can be currently allocated from the heap.
- [size](https://developer.apple.com/documentation/metal/mtlheap/size) — The total size of the heap, in bytes.
- [usedSize](https://developer.apple.com/documentation/metal/mtlheap/usedsize) — The size of all resources currently in the heap, in bytes.
- [currentAllocatedSize](https://developer.apple.com/documentation/metal/mtlheap/currentallocatedsize) — The size, in bytes, of the current heap allocation.

### Checking a heap’s permanent configuration
- [device](https://developer.apple.com/documentation/metal/mtlheap/device) — The device object that created the heap.
- [type](https://developer.apple.com/documentation/metal/mtlheap/type) — The heap’s type.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) — The heap’s storage mode.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) — The heap’s CPU cache mode.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheap/hazardtrackingmode) — The heap’s hazard tracking mode.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheap/resourceoptions) — The options for resources created by the heap.

## See also

### Resource memory allocation and management
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) — A configuration that customizes the behavior for a Metal memory heap.
- [MTLHeapType](https://developer.apple.com/documentation/metal/mtlheaptype) — The options you use to choose the heap type.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.
