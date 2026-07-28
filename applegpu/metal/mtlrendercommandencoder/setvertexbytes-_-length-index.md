# setVertexBytes(_:length:index:)

*Instance Method · iOS 8.3, iPadOS 8.3, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:index:)>

Creates a buffer from bytes and assigns it to an entry in the vertex shader argument table.

## Declaration

```swift
func setVertexBytes(_ bytes: UnsafeRawPointer, length: Int, index: Int)
```

## Parameters

- **bytes** — A pointer to argument data the method copies to an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) and assigns to an entry in the vertex shader argument table for buffers.
- **length** — The number of bytes the method copies from the `bytes` pointer.
- **index** — An integer that represents the entry in the vertex shader argument table for buffers that stores a record of the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) the method creates from `bytes`.

## Discussion

The method is equivalent to creating an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains the same data as `bytes` and calling the [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) method. However, this method avoids the overhead of creating a buffer to store your data; instead, Metal manages the data.

> **Important:**
>  Only call this method for single-use data that’s smaller than 4 KB.

For data that’s more than 4 KB, create an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance and pass it to [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)).

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) — Assigns a buffer to an entry in the vertex shader argument table.
- [setVertexBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:attributestride:index:))
- [setVertexBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the vertex shader argument table.
- [setVertexBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:attributestrides:range:))
- [setVertexBytes(_:length:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:attributestride:index:))
- [setVertexBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(_:index:)) — Updates an entry in the vertex shader argument table with a new location within the entry’s current buffer.
- [setVertexBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(offset:attributestride:index:))
