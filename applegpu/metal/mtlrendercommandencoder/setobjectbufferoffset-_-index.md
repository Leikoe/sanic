# setObjectBufferOffset(_:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbufferoffset(_:index:)>

Updates an entry in the object shader argument table with a new location within the entry’s current buffer.

## Declaration

```swift
func setObjectBufferOffset(_ offset: Int, index: Int)
```

## Parameters

- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the object shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the object shader argument table for buffers that already stores a record of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).

## Discussion

The command this method encodes changes the offset for a mesh buffer that already has a previous assignment from one of your earlier commands.

For more information, see:

- [setObjectBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:))

- [setObjectBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers(_:offsets:range:)) (Swift)

- [setObjectBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers:offsets:withrange:) (Objective-C)

The command can also adjust the offset for an entry that you previously set with the [setObjectBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbytes(_:length:index:)) method.

> **Tip:**
>  If you’re only updating an offset, this method is typically more efficient than rebinding a buffer or byte block with the methods above.

## See also

### Assigning buffers for object shaders
- [setObjectBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffer(_:offset:index:)) — Assigns a buffer to an entry in the object shader argument table.
- [setObjectBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the object shader argument table.
- [setObjectBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setobjectbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the object shader argument table.
