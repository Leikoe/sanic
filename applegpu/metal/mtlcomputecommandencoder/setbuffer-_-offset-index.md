# setBuffer(_:offset:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:index:)>

Binds a buffer to the buffer argument table, allowing compute kernels to access its data on the GPU.

## Declaration

```swift
func setBuffer(_ buffer: (any MTLBuffer)?, offset: Int, index: Int)
```

## Parameters

- **buffer** — The [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance to bind to the argument table.
- **offset** — The number of bytes to skip in the buffer before the first element of data.
- **index** — The index the buffer binds to in the argument table.

## Discussion

For buffers binding to an argument using the `device` address space, align the offset to the data type’s size. The maximum size for an offset is `16` bytes.

For buffers in the `constant` address space, the minimum alignment depends on the hardware running your app. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for information on each Apple GPU family.

Rebinding an already bound buffer causes a Metal error.

## See also

### Binding buffers
- [setBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:attributestride:index:)) — Binds a buffer with a stride to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:range:)) — Binds multiple buffers to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:attributestrides:range:)) — Binds multiple buffers with data in stride to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(_:index:)) — Changes where the data begins in a buffer already bound to the buffer argument table.
- [setBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(offset:attributestride:index:)) — Changes where the data begins and the distance between adjacent elements in a buffer already bound to the buffer argument table.
