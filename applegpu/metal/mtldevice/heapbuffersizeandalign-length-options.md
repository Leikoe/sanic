# heapBufferSizeAndAlign(length:options:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)>

Returns the size and alignment, in bytes, of a buffer if you create it from a heap.

## Declaration

```swift
func heapBufferSizeAndAlign(length: Int, options: MTLResourceOptions = []) -> MTLSizeAndAlign
```

## Parameters

- **length** — The size of the buffer, in bytes.
- **options** — An [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) instance for a would-be buffer’s storage and hazard tracking modes. See [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals) and [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes) for more information.

## Return Value

An [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) instance.

## Discussion

Use this method to help estimate an appropriate size for a new heap before you create it.

## See also

### Working with resource heaps
- [makeHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)) — Creates a new GPU heap instance.
- [heapTextureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heaptexturesizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of a texture if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(size:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(size:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap with a descriptor.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.
