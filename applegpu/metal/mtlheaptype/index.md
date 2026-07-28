# MTLHeapType

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheaptype>

The options you use to choose the heap type.

## Declaration

```swift
enum MTLHeapType
```

## Topics

### Specifying the heap type
- [MTLHeapType.automatic](https://developer.apple.com/documentation/metal/mtlheaptype/automatic) — A heap that automatically places new resource allocations.
- [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement) — The app controls placement of resources on the heap.
- [MTLHeapType.sparse](https://developer.apple.com/documentation/metal/mtlheaptype/sparse) — The heap contains sparse texture tiles.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlheaptype/init(rawvalue:))

## See also

### Resource memory allocation and management
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) — A memory pool from which you can suballocate resources.
- [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) — A configuration that customizes the behavior for a Metal memory heap.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.
