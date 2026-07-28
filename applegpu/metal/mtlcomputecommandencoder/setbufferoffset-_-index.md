# setBufferOffset(_:index:)

*Instance Method · iOS 8.3, iPadOS 8.3, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(_:index:)>

Changes where the data begins in a buffer already bound to the buffer argument table.

## Declaration

```swift
func setBufferOffset(_ offset: Int, index: Int)
```

## Parameters

- **offset** — Where the data to bind begins, in bytes, from the start of the bound buffer.
- **index** — The argument table entry to change.

## Discussion

Prefer calling this method to unbinding and then rebinding data.

For buffers binding to an argument using the `device` address space, align the offset to the data type’s size. The maximum size for an offset is `16` bytes.

For buffers in the `constant` address space, the minimum alignment depends on the hardware running your app. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for information on each Apple GPU family.

## See also

### Binding buffers
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:index:)) — Binds a buffer to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:attributestride:index:)) — Binds a buffer with a stride to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:range:)) — Binds multiple buffers to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:attributestrides:range:)) — Binds multiple buffers with data in stride to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(offset:attributestride:index:)) — Changes where the data begins and the distance between adjacent elements in a buffer already bound to the buffer argument table.
