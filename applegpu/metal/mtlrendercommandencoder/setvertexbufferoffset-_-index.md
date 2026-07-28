# setVertexBufferOffset(_:index:)

*Instance Method · iOS 8.3, iPadOS 8.3, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(_:index:)>

Updates an entry in the vertex shader argument table with a new location within the entry’s current buffer.

## Declaration

```swift
func setVertexBufferOffset(_ offset: Int, index: Int)
```

## Parameters

- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the vertex shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the vertex shader argument table for buffers that already stores a record of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).

## Discussion

The command this method encodes changes the offset for a fragment buffer that already has a previous assignment from one of your earlier commands.

For more information, see:

- [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:))

- [setVertexBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:range:)) (Swift)

- [setVertexBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers:offsets:withrange:) (Objective-C)

The command can also adjust the offset for an entry that you previously set with the [setVertexBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:index:)) method.

> **Tip:**
>  If you’re only updating an offset, this method is typically more efficient than rebinding a buffer or byte block with the methods above.

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) — Assigns a buffer to an entry in the vertex shader argument table.
- [setVertexBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:attributestride:index:))
- [setVertexBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the vertex shader argument table.
- [setVertexBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:attributestrides:range:))
- [setVertexBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the vertex shader argument table.
- [setVertexBytes(_:length:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:attributestride:index:))
- [setVertexBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(offset:attributestride:index:))
