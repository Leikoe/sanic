# makeHeap(descriptor:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)>

Creates a new GPU heap instance.

## Declaration

```swift
func makeHeap(descriptor: MTLHeapDescriptor) -> (any MTLHeap)?
```

## Parameters

- **descriptor** — An [MTLHeapDescriptor](https://developer.apple.com/documentation/metal/mtlheapdescriptor) instance.

## Return Value

A new [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instance if the method completed successfully; otherwise nil.

## Discussion

For more information about using heaps, see [Memory heaps](https://developer.apple.com/documentation/metal/memory-heaps).

## See also

### Working with resource heaps
- [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)) — Returns the size and alignment, in bytes, of a buffer if you create it from a heap.
- [heapTextureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heaptexturesizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of a texture if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(size:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(size:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap with a descriptor.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.
