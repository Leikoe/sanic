# setFragmentBufferOffset(_:index:)

*Instance Method · iOS 8.3, iPadOS 8.3, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbufferoffset(_:index:)>

Updates an entry in the fragment shader argument table with a new location within the entry’s current buffer.

## Declaration

```swift
func setFragmentBufferOffset(_ offset: Int, index: Int)
```

## Parameters

- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the fragment shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the fragment shader argument table for buffers that already stores a record of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).

## Discussion

The command this method encodes changes the offset for a fragment buffer that already has a previous assignment from one of your earlier commands.

For more information, see:

- [setFragmentBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffer(_:offset:index:))

- [setFragmentBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffers(_:offsets:range:)) (Swift)

- [setFragmentBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffers:offsets:withrange:) (Objective-C)

The command can also adjust the offset for an entry that you previously set with the [setFragmentBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbytes(_:length:index:)) method.

> **Tip:**
>  If you’re only updating an offset, this method is typically more efficient than rebinding a buffer or byte block with the methods above.

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setFragmentBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffer(_:offset:index:)) — Assigns a buffer to an entry in the fragment shader argument table.
- [setFragmentBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the fragment shader argument table.
- [setFragmentBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the fragment shader argument table.
