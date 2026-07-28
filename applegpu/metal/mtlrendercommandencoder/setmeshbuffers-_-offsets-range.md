# setMeshBuffers(_:offsets:range:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffers(_:offsets:range:)>

Assigns multiple buffers to a range of entries in the mesh shader argument table.

## Declaration

```swift
func setMeshBuffers(_ buffers: [(any MTLBuffer)?], offsets: [Int], range: Range<Int>)
```

## Parameters

- **buffers** — An array of [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances the command assigns to entries in the mesh shader argument table for buffers.
- **offsets** — An array of integers. Each element represents the location, in bytes, from the start of the corresponding [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) element in `buffers` where the mesh shader argument data begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **range** — A span of integers that represent the entries in the mesh shader argument table for buffers. Each entry stores a record of the corresponding element in `buffers` and `offsets`.

## Discussion

By default, the texture at each index is `nil`.

> **Note:**
>  The Objective-C version of this method is [setMeshBuffers:offsets:withRange:](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffers:offsets:withrange:).

## See also

### Assigning buffers for mesh shaders
- [setMeshBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffer(_:offset:index:)) — Assigns a buffer to an entry in the mesh shader argument table.
- [setMeshBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbytes(_:length:index:)) — Creates a buffer from bytes and assigns it to an entry in the mesh shader argument table.
- [setMeshBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbufferoffset(_:index:)) — Updates an entry in the mesh shader argument table with a new location within the entry’s current buffer.
