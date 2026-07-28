# heapAccelerationStructureSizeAndAlign(descriptor:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(descriptor:)>

Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap with a descriptor.

## Declaration

```swift
func heapAccelerationStructureSizeAndAlign(descriptor: MTLAccelerationStructureDescriptor) -> MTLSizeAndAlign
```

## Parameters

- **descriptor** — An [MTLAccelerationStructureDescriptor](https://developer.apple.com/documentation/metal/mtlaccelerationstructuredescriptor) instance.

## Return Value

An [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) instance.

## Discussion

Use this method to help estimate an appropriate size for a new heap before you create it.

## See also

### Working with resource heaps
- [makeHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeheap(descriptor:)) — Creates a new GPU heap instance.
- [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)) — Returns the size and alignment, in bytes, of a buffer if you create it from a heap.
- [heapTextureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heaptexturesizeandalign(descriptor:)) — Returns the size and alignment, in bytes, of a texture if you create it from a heap.
- [heapAccelerationStructureSizeAndAlign(size:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(size:)) — Returns the size and alignment, in bytes, of an acceleration structure if you create it from a heap.
- [MTLSizeAndAlign](https://developer.apple.com/documentation/metal/mtlsizeandalign) — The size and alignment of a resource, in bytes.
