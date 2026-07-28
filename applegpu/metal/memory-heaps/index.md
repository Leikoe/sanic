# Memory heaps

*API Collection*

<https://developer.apple.com/documentation/metal/memory-heaps>

Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.

## Overview

Use an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) to quickly create and destroy GPU resources. Heaps can also help your apps save memory by aliasing portions of it in multiple places.

Create a heap by calling an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)) method.

> **Note:**
>  Metal only synchronizes resources that you create from a Metal heap and that have the [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheap/hazardtrackingmode) property set to [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked).

## Topics

### Resource memory allocation and management
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) — A memory pool from which you can suballocate resources.
- [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) — A configuration that customizes the behavior for a Metal memory heap.
- [MTLHeapType](https://developer.apple.com/documentation/metal/mtlheaptype) — The options you use to choose the heap type.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.

## See also

### Resources
- [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals) — Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
- [Buffers](https://developer.apple.com/documentation/metal/buffers) — Create and manage untyped data your app uses to exchange information with its shader functions.
- [Textures](https://developer.apple.com/documentation/metal/textures) — Create and manage typed data your app uses to exchange information with its shader functions.
- [Resource loading](https://developer.apple.com/documentation/metal/resource-loading) — Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.
- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) — Prevent multiple commands that can access the same resources simultaneously by coordinating those reads and writes with barriers, fences, or events.
