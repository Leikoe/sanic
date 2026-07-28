# setObjectBuffer(_:offset:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:)>

Assigns a buffer to an entry in the object shader argument table.

## Declaration

```swift
func setObjectBuffer(_ buffer: (any MTLBuffer)?, offset: Int, index: Int)
```

## Parameters

- **buffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance the command assigns to an entry in the object shader argument table for buffers.
- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the object shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the object shader argument table for buffers that stores a record of `buffer` and `offset`.

## Discussion

By default, the texture at each index is `nil`.

## See also

### Assigning buffers for object shaders
- [setObjectBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the object shader argument table.
- [setObjectBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the object shader argument table.
- [setObjectBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbufferoffset(_:index:)) — Updates an entry in the object shader argument table with a new location within the entry’s current buffer.
