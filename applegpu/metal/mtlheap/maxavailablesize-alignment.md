# maxAvailableSize(alignment:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/maxavailablesize(alignment:)>

The maximum size of a resource, in bytes, that can be currently allocated from the heap.

## Declaration

```swift
func maxAvailableSize(alignment: Int) -> Int
```

## Parameters

- **alignment** — The alignment of the resource, in bytes. This value needs to be a power of two.

## Return Value

The maximum size for the resource, in bytes.

## Discussion

This method measures fragmentation within the heap. You can use the [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)) and [heapTextureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heaptexturesizeandalign(descriptor:)) methods to help you determine the correct alignment for the resource.

## See also

### Checking a heap’s size information
- [size](https://developer.apple.com/documentation/metal/mtlheap/size) — The total size of the heap, in bytes.
- [usedSize](https://developer.apple.com/documentation/metal/mtlheap/usedsize) — The size of all resources currently in the heap, in bytes.
- [currentAllocatedSize](https://developer.apple.com/documentation/metal/mtlheap/currentallocatedsize) — The size, in bytes, of the current heap allocation.
