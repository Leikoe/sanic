# setBytes(_:length:attributeStride:index:)

*Instance Method · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbytes(_:length:attributestride:index:)>

Copies data with a given stride directly to the GPU to populate an entry in the buffer argument table.

## Declaration

```swift
func setBytes(_ bytes: UnsafeRawPointer, length: Int, attributeStride stride: Int, index: Int)
```

## Parameters

- **bytes** — A pointer to the memory where the data to copy starts.
- **length** — The number of bytes to copy.
- **stride** — The number of bytes between the start of one element and the start of the next.
- **index** — The index the data binds to in the argument table.

## Discussion

> **Important:**
>  This method only works for data smaller than 4 kilobytes that doesn’t persist. Create an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance if your data exceeds 4 KB, needs to persist on the GPU, or you access results on the CPU.

This method allows Metal to copy data directly onto the GPU, rather than creating a new [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance and binding it. Binding data directly can improve performance, especially when making many small allocations.

## See also

### Binding raw bytes
- [setBytes(_:length:index:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setbytes(_:length:index:)) — Copies data directly to the GPU to populate an entry in the buffer argument table.
