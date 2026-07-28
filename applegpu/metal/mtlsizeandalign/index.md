# MTLSizeAndAlign

*Structure · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlsizeandalign>

The size and alignment of a resource, in bytes.

## Declaration

```swift
struct MTLSizeAndAlign
```

## Topics

### Accessing the size and alignment
- [size](https://developer.apple.com/documentation/metal/mtlsizeandalign/size) — The size of a resource, in bytes.
- [align](https://developer.apple.com/documentation/metal/mtlsizeandalign/align) — The alignment of a resource, in bytes.

### Creating instances
- [init()](https://developer.apple.com/documentation/metal/mtlsizeandalign/init()) — Creates a default instance.
- [init(size:align:)](https://developer.apple.com/documentation/metal/mtlsizeandalign/init(size:align:)) — Creates a new instance initialized to the given values.

## See also

### Resource memory allocation and management
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Implementing a multistage image filter using heaps and fences](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences) — Use fences to synchronize access to resources allocated on a heap.
- [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) — A memory pool from which you can suballocate resources.
- [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) — A configuration that customizes the behavior for a Metal memory heap.
- [MTLHeapType](https://developer.apple.com/documentation/metal/mtlheaptype) — The options you use to choose the heap type.
