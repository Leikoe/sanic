# setBuffers(_:offsets:range:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:range:)>

Binds multiple buffers to the buffer argument table at once, allowing compute kernels to access their data on the GPU.

## Declaration

```swift
func setBuffers(_ buffers: [(any MTLBuffer)?], offsets: [Int], range: Range<Int>)
```

## Parameters

- **buffers** — The [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instances to bind to the buffer argument table.
- **offsets** — An array of offsets, each of which specifies where the data begins, in bytes, from the start of its corresponding buffer.
- **range** — The argument table indices to bind each of the `buffers` to, in the order they appear.

## Discussion

> **Important:**
>  This method requires that the length of `buffers` and `offsets` are equal to the length of `range`.

For buffers binding to an argument using the `device` address space, align the offset to the data type’s size. The maximum size for an offset is `16` bytes.

For buffers in the `constant` address space, the minimum alignment depends on the hardware running your app. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for information on each Apple GPU family.

Rebinding an already bound buffer causes a Metal error.

## See also

### Binding buffers
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:index:)) — Binds a buffer to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffer(_:offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffer(_:offset:attributestride:index:)) — Binds a buffer with a stride to the buffer argument table, allowing compute kernels to access its data on the GPU.
- [setBuffers(_:offsets:attributeStrides:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbuffers(_:offsets:attributestrides:range:)) — Binds multiple buffers with data in stride to the buffer argument table at once, allowing compute kernels to access their data on the GPU.
- [setBufferOffset(_:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(_:index:)) — Changes where the data begins in a buffer already bound to the buffer argument table.
- [setBufferOffset(offset:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbufferoffset(offset:attributestride:index:)) — Changes where the data begins and the distance between adjacent elements in a buffer already bound to the buffer argument table.
