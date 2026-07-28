# makeBuffer(length:options:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/makebuffer(length:options:)>

Creates a buffer on the heap.

## Declaration

```swift
func makeBuffer(length: Int, options: MTLResourceOptions = []) -> (any MTLBuffer)?
```

## Parameters

- **length** — The size, in bytes, of the buffer.
- **options** — Options that describe the properties of the buffer.

## Return Value

A new buffer object backed by heap memory, or `nil` if the heap memory is full.

## Discussion

You can call the method with the following restrictions:

- The heap’s type needs to be [MTLHeapType.automatic](https://developer.apple.com/documentation/metal/mtlheaptype/automatic)

- The buffer’s storage mode option needs to match the heap’s [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) property

- The buffer’s CPU cache mode option needs to match the heap’s [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) property

## See also

### Creating buffers from a heap
- [makeBuffer(length:options:offset:)](https://developer.apple.com/documentation/metal/mtlheap/makebuffer(length:options:offset:)) — Creates a buffer at a specified offset on the heap.
