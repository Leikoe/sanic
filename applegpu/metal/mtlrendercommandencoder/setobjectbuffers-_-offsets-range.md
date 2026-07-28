# setObjectBuffers(_:offsets:range:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers(_:offsets:range:)>

Assigns multiple buffers to a range of entries in the object shader argument table.

## Declaration

```swift
func setObjectBuffers(_ buffers: [(any MTLBuffer)?], offsets: [Int], range: Range<Int>)
```

## Parameters

- **buffers** — An array of [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances the command assigns to entries in the object shader argument table for buffers.
- **offsets** — An array of integers. Each element represents the location, in bytes, from the start of the corresponding [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) element in `buffers` where the object shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **range** — A span of integers that represent the entries in the object shader argument table for buffers. Each entry stores a record of the corresponding element in `buffers` and `offsets`.

## Discussion

By default, the texture at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setObjectBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers:offsets:withrange:).

## See also

### Assigning buffers for object shaders
- [setObjectBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:)) — Assigns a buffer to an entry in the object shader argument table.
- [setObjectBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the object shader argument table.
- [setObjectBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbufferoffset(_:index:)) — Updates an entry in the object shader argument table with a new location within the entry’s current buffer.
