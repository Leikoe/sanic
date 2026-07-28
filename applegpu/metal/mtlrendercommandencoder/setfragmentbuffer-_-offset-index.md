# setFragmentBuffer(_:offset:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffer(_:offset:index:)>

Assigns a buffer to an entry in the fragment shader argument table.

## Declaration

```swift
func setFragmentBuffer(_ buffer: (any MTLBuffer)?, offset: Int, index: Int)
```

## Parameters

- **buffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance the command assigns to an entry in the fragment shader argument table for buffers.
- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the fragment shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the fragment shader argument table for buffers that stores a record of `buffer` and `offset`.

## Discussion

By default, the buffer at each index is `nil`.

## See also

### Assigning buffers
- [setFragmentBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the fragment shader argument table.
- [setFragmentBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the fragment shader argument table.
- [setFragmentBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setfragmentbufferoffset(_:index:)) — Updates an entry in the fragment shader argument table with a new location within the entry’s current buffer.
