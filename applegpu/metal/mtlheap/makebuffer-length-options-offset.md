# makeBuffer(length:options:offset:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/makebuffer(length:options:offset:)>

Creates a buffer at a specified offset on the heap.

## Declaration

```swift
func makeBuffer(length: Int, options: MTLResourceOptions = [], offset: Int) -> (any MTLBuffer)?
```

## Parameters

- **length** — The size of the buffer, in bytes.
- **options** — Options that describe the properties of the buffer.
- **offset** — The distance, in bytes, to place the buffer relative to the start of the heap.

## Return Value

A new buffer, or `nil` if the heap is not a placement heap.

## Discussion

You can call the method with the following restrictions:

- The heap’s type needs to be [MTLHeapType.placement](https://developer.apple.com/documentation/metal/mtlheaptype/placement)

- The buffer’s storage mode option needs to match the heap’s [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) property

- The buffer’s CPU cache mode option needs to match the heap’s [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) property

Use the [heapBufferSizeAndAlign(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/heapbuffersizeandalign(length:options:)) method to determine the required size and alignment. If you don’t align the buffer correctly or it extends past the end of the heap, the behavior is undefined.

> **Note:**
>  The new buffer can implicitly alias the underlying memory of other resources already in the heap within the overlapping half-open range of `[offset, offset + requiredSize)`.

## See also

### Creating buffers from a heap
- [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtlheap/makebuffer(length:options:)) — Creates a buffer on the heap.
