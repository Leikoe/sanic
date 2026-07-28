# setVertexBuffers(_:offsets:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:range:)>

Assigns multiple buffers to a range of entries in the vertex shader argument table.

## Declaration

```swift
func setVertexBuffers(_ buffers: [(any MTLBuffer)?], offsets: [Int], range: Range<Int>)
```

## Parameters

- **buffers** — An array of [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances the command assigns to entries in the vertex shader argument table for buffers.
- **offsets** — An array of integers. Each element represents the location, in bytes, from the start of the corresponding [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) element in `buffers` where the vertex shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **range** — A span of integers that represent the entries in the vertex shader argument table for buffers. Each entry stores a record of the corresponding element in `buffers` and `offsets`.

## Discussion

By default, the buffer at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setVertexBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers:offsets:withrange:).

## See also

### Assigning buffers
- [setVertexBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:index:)) — Assigns a buffer to an entry in the vertex shader argument table.
- [setVertexBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffer(_:offset:attributestride:index:))
- [setVertexBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbuffers(_:offsets:attributestrides:range:))
- [setVertexBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the vertex shader argument table.
- [setVertexBytes(_:length:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbytes(_:length:attributestride:index:))
- [setVertexBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(_:index:)) — Updates an entry in the vertex shader argument table with a new location within the entry’s current buffer.
- [setVertexBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setvertexbufferoffset(offset:attributestride:index:))
