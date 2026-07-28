# sparsePageSize

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheapdescriptor/sparsepagesize>

The page size for any resources you allocate from the heaps you create with this descriptor.

## Declaration

```swift
var sparsePageSize: MTLSparsePageSize { get set }
```

## Discussion

This property’s default value is 16 kilobytes ([MTLSparsePageSize.size16](https://developer.apple.com/documentation/metal/mtlsparsepagesize/size16)), which is a smaller page size option that can help reduce your app’s memory usage. However, you can reduce operational overhead for sparse textures with larger page sizes, such as [MTLSparsePageSize.size64](https://developer.apple.com/documentation/metal/mtlsparsepagesize/size64) and [MTLSparsePageSize.size256](https://developer.apple.com/documentation/metal/mtlsparsepagesize/size256). These operations include blit commands and the configuration of sparse texture mappings (see [Blit passes](https://developer.apple.com/documentation/metal/blit-passes) and [MTLResourceStateCommandEncoder](https://developer.apple.com/documentation/metal/mtlresourcestatecommandencoder), respectively).

## See also

### Configuring a heap
- [type](https://developer.apple.com/documentation/metal/mtlheapdescriptor/type) — The memory placement strategy for any resources you allocate from the heaps you create with this descriptor.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/storagemode) — The storage mode for the heaps you create with this descriptor.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/cpucachemode) — The CPU cache behavior for any resources you allocate from the heaps you create with this descriptor.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheapdescriptor/hazardtrackingmode) — The hazard tracking behavior for any resources you allocate from the heaps you create with this descriptor.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheapdescriptor/resourceoptions) — The combined behavior for any resources you allocate from the heaps you create with this descriptor.
- [size](https://developer.apple.com/documentation/metal/mtlheapdescriptor/size) — The total amount of memory, in bytes, for the heaps you create with this descriptor.
