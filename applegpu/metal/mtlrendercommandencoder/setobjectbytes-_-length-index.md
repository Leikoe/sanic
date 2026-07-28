# setObjectBytes(_:length:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbytes(_:length:index:)>

Creates a buffer from bytes and assigns it to an entry in the object shader argument table.

## Declaration

```swift
func setObjectBytes(_ bytes: UnsafeRawPointer, length: Int, index: Int)
```

## Parameters

- **bytes** — A pointer to argument data the method copies to an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) and assigns to an entry in the object shader argument table for buffers.
- **length** — The number of bytes the method copies from the `bytes` pointer.
- **index** — An integer that represents the entry in the object shader argument table for buffers that stores a record of the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) the method creates from `bytes`.

## Discussion

The method is equivalent to creating an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains the same data as `bytes` and calling the [setObjectBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbufferoffset(_:index:)) method. However, this method avoids the overhead of creating a buffer to store your data; instead, Metal manages the data.

> **Important:**
>  Only call this method for single-use data that’s smaller than 4 KB.

For data that’s more than 4 KB, create an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance and pass it to [setObjectBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:)).

## See also

### Assigning buffers for object shaders
- [setObjectBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:)) — Assigns a buffer to an entry in the object shader argument table.
- [setObjectBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the object shader argument table.
- [setObjectBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbufferoffset(_:index:)) — Updates an entry in the object shader argument table with a new location within the entry’s current buffer.
