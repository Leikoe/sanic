# size

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor/size>

The total amount of memory, in bytes, for the heaps you create with this descriptor.

## Declaration

```swift
var size: Int { get set }
```

## Discussion

You can use various [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods to help you estimate an appropriate heap size, including the following:

- [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:))

- [heapTextureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heaptexturesizeandalign(descriptor:))

- [heapAccelerationStructureSizeAndAlign(size:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(size:))

- [heapAccelerationStructureSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/heapaccelerationstructuresizeandalign(descriptor:))

> **Note:**
>  Metal may round a heap’s size to a page boundary.

This property’s default value is `0`.

## See also

### Configuring a heap
- [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) — The memory placement strategy for any resources you allocate from the heaps you create with this descriptor.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/storagemode) — The storage mode for the heaps you create with this descriptor.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/cpucachemode) — The CPU cache behavior for any resources you allocate from the heaps you create with this descriptor.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) — The hazard tracking behavior for any resources you allocate from the heaps you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheapdescriptor/resourceoptions) — The combined behavior for any resources you allocate from the heaps you create with this descriptor.
- [sparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/sparsepagesize) — The page size for any resources you allocate from the heaps you create with this descriptor.
