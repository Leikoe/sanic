# makeBuffer(length:options:placementSparsePageSize:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:placementsparsepagesize:)>

Creates a new placement sparse buffer of a specific length.

## Declaration

```swift
func makeBuffer(length: Int, options: MTLResourceOptions = [], placementSparsePageSize: MTLSparsePageSize) -> (any MTLBuffer)?
```

## Parameters

- **length** — The size of the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), in bytes.
- **options** — A [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) instance that establishes the buffer’s storage modes.
- **placementSparsePageSize** — [MTLSparsePageSize](https://developer.apple.com/documentation/metal/mtlsparsepagesize) to use for the placement sparse buffer.

## Return Value

A [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance, or `nil` if the function failed.

## Discussion

This method creates a new placement sparse [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) of a specific length. You assign memory to placement sparse buffers using a [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) of type [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement).
