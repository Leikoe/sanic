# setTileBufferOffset(_:index:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebufferoffset(_:index:)>

Updates an entry in the tile shader argument table with a new location within the entry’s current buffer.

## Declaration

```swift
func setTileBufferOffset(_ offset: Int, index: Int)
```

## Parameters

- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the tile shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the tile shader argument table for buffers that already stores a record of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).

## Discussion

The command this method encodes changes the offset for a fragment buffer that already has a previous assignment from one of your earlier commands.

For more information, see:

- [setTileBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:))

- [setTileBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffers(_:offsets:range:)) (Swift)

- [setTileBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffers:offsets:withrange:) (Objective-C)

The command can also adjust the offset for an entry that you previously set with the [setTileBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebytes(_:length:index:)) method.

> **Tip:**
>  If you’re only updating an offset, this method is typically more efficient than rebinding a buffer or byte block with the methods above.

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setTileBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffer(_:offset:index:)) — Assigns a buffer to an entry in the tile shader argument table.
- [setTileBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the tile shader argument table.
- [setTileBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/settilebytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the tile shader argument table.
