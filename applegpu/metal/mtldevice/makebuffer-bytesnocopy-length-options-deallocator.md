# makeBuffer(bytesNoCopy:length:options:deallocator:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytesnocopy:length:options:deallocator:)>

Creates a buffer that wraps an existing contiguous memory allocation.

## Declaration

```swift
func makeBuffer(bytesNoCopy pointer: UnsafeMutableRawPointer, length: Int, options: MTLResourceOptions = [], deallocator: (@Sendable (UnsafeMutableRawPointer, Int) -> Void)? = nil) -> (any MTLBuffer)?
```

## Parameters

- **pointer** — A page-aligned pointer to the starting memory address.
- **length** — The size of the new buffer, in bytes, that results in a page-aligned region of memory.
- **options** — An [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) instance that sets the buffer’s storage and hazard-tracking modes. See [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals) and [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes) for more information.
- **deallocator** — A block the framework invokes when it deallocates the buffer so that your app can release the underlying memory; otherwise `nil` to opt out.

## Return Value

A new [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance if the method completes successfully; otherwise `nil`.

## Discussion

> **Important:**
>  The existing memory allocation needs to exist within a single virtual memory (VM) region.

## See also

### Creating buffers
- [maxBufferLength](https://developer.apple.com/documentation/metal/mtldevice/maxbufferlength) — The largest amount of memory, in bytes, that a GPU device can allocate to a buffer instance.
- [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:)) — Creates a buffer the method clears with zero values.
- [makeBuffer(bytes:length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytes:length:options:)) — Allocates a new buffer of a given length and initializes its contents by copying existing data into it.
