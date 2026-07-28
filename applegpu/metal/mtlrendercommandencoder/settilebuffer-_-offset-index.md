# setTileBuffer(_:offset:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:)>

Assigns a buffer to an entry in the tile shader argument table.

## Declaration

```swift
func setTileBuffer(_ buffer: (any MTLBuffer)?, offset: Int, index: Int)
```

## Parameters

- **buffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance the command assigns to an entry in the tile shader argument table for buffers.
- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the tile shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the tile shader argument table for buffers that stores a record of `buffer` and `offset`.

## Discussion

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setTileBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the tile shader argument table.
- [setTileBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the tile shader argument table.
- [setTileBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebufferoffset(_:index:)) — Updates an entry in the tile shader argument table with a new location within the entry’s current buffer.
