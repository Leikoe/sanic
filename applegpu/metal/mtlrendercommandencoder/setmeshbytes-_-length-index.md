# setMeshBytes(_:length:index:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbytes(_:length:index:)>

Creates a buffer from bytes and assigns it to an entry in the mesh shader argument table.

## Declaration

```swift
func setMeshBytes(_ bytes: UnsafeRawPointer, length: Int, index: Int)
```

## Parameters

- **bytes** — A pointer to argument data the method copies to an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) and assigns to an entry in the mesh shader argument table for buffers.
- **length** — The number of bytes the method copies from the `bytes` pointer.
- **index** — An integer that represents the entry in the mesh shader argument table for buffers that stores a record of the [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) the method creates from `bytes`.

## Discussion

The method is equivalent to creating an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance that contains the same data as `bytes` and calling the [setMeshBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffer(_:offset:index:)) method. However, this method avoids the overhead of creating a buffer to store your data; instead, Metal manages the data.

> **Important:**
>  Only call this method for single-use data that’s smaller than 4 KB.

For data that’s more than 4 KB, create an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance and pass it to [setMeshBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbufferoffset(_:index:)).

## See also

### Assigning buffers for mesh shaders
- [setMeshBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffer(_:offset:index:)) — Assigns a buffer to an entry in the mesh shader argument table.
- [setMeshBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbuffers(_:offsets:range:)) — Assigns multiple buffers to a range of entries in the mesh shader argument table.
- [setMeshBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setmeshbufferoffset(_:index:)) — Updates an entry in the mesh shader argument table with a new location within the entry’s current buffer.
