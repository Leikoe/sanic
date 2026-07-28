# updateMappings(buffer:heap:operations:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandqueue/updatemappings(buffer:heap:operations:)>

Updates multiple regions within a placement sparse buffer to alias specific tiles from a Metal heap.

## Declaration

```swift
func updateMappings(buffer: any MTLBuffer, heap: (any MTLHeap)?, operations: [MTL4UpdateSparseBufferMappingOperation])
```

## Parameters

- **buffer** — A placement sparse [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).
- **heap** — An [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) you allocate with type [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement).
- **operations** — An array of [MTL4UpdateSparseBufferMappingOperation](https://developer.apple.com/documentation/metal/mtl4updatesparsebuffermappingoperation) instances to perform.

## Discussion

You can provide a `nil` parameter to the `heap` argument only when you perform unmap operations. Otherwise, you are responsible for ensuring parameter `heap` references an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) that has a [maxCompatiblePlacementSparsePageSize](https://developer.apple.com/documentation/metal/mtlheapdescriptor/maxcompatibleplacementsparsepagesize) of at least the buffer’s `placementSparsePageSize` you assign when creating the sparse buffer via [makeBuffer(length:options:placementSparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:placementsparsepagesize:)).
