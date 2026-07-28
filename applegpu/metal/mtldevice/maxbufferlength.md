# maxBufferLength

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/maxbufferlength>

The largest amount of memory, in bytes, that a GPU device can allocate to a buffer instance.

## Declaration

```swift
var maxBufferLength: Int { get }
```

## Discussion

The property’s value is at least 256 MB (268,435,456 bytes).

## See also

### Creating buffers
- [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:)) — Creates a buffer the method clears with zero values.
- [makeBuffer(bytes:length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytes:length:options:)) — Allocates a new buffer of a given length and initializes its contents by copying existing data into it.
- [makeBuffer(bytesNoCopy:length:options:deallocator:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytesnocopy:length:options:deallocator:)) — Creates a buffer that wraps an existing contiguous memory allocation.
