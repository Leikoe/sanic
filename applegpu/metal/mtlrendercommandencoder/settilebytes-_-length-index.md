# setTileBytes(_:length:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebytes(_:length:index:)>

Creates a buffer from bytes and assigns it to an entry in the tile shader argument table.

## Declaration

```swift
func setTileBytes(_ bytes: UnsafeRawPointer, length: Int, index: Int)
```

## Parameters

- **bytes** — A pointer to argument data the method copies to an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) and assigns to an entry in the tile shader argument table for buffers.
- **length** — The number of bytes the method copies from the `bytes` pointer.
- **index** — An integer that represents the entry in the tile shader argument table for buffers that stores a record of the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) the method creates from `bytes`.

## Discussion

The method is equivalent to creating an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains the same data as `bytes` and calling the [setTileBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:)) method. However, this method avoids the overhead of creating a buffer to store your data; instead, Metal manages the data.

> **Important:**
>  Only call this method for single-use data that’s smaller than 4 KB.

For data that’s more than 4 KB, create an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance and pass it to [setTileBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:)).

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setTileBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:)) — Assigns a buffer to an entry in the tile shader argument table.
- [setTileBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the tile shader argument table.
- [setTileBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebufferoffset(_:index:)) — Updates an entry in the tile shader argument table with a new location within the entry’s current buffer.
