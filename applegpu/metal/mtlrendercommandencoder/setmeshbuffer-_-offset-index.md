# setMeshBuffer(_:offset:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffer(_:offset:index:)>

Assigns a buffer to an entry in the mesh shader argument table.

## Declaration

```swift
func setMeshBuffer(_ buffer: (any MTLBuffer)?, offset: Int, index: Int)
```

## Parameters

- **buffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance the command assigns to an entry in the mesh shader argument table for buffers.
- **offset** — An integer that represents the location, in bytes, from the start of `buffer` where the mesh shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **index** — An integer that represents the entry in the mesh shader argument table for buffers that stores a record of `buffer` and `offset`.

## Discussion

By default, the texture at each index is `nil`.

## See also

### Assigning buffers for mesh shaders
- [setMeshBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the mesh shader argument table.
- [setMeshBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the mesh shader argument table.
- [setMeshBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbufferoffset(_:index:)) — Updates an entry in the mesh shader argument table with a new location within the entry’s current buffer.
