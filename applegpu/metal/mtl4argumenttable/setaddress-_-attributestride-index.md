# setAddress(_:attributeStride:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4argumenttable/setaddress(_:attributestride:index:)>

Binds a GPU address to a buffer binding slot, providing a dynamic vertex stride.

## Declaration

```swift
func setAddress(_ gpuAddress: MTLGPUAddress, attributeStride stride: Int, index bindingIndex: Int)
```

## Parameters

- **gpuAddress** — The GPU address of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) to set.
- **stride** — The stride between attributes in the buffer.
- **bindingIndex** — A valid binding index in the buffer binding range. It is an error for this value to match or exceed the value of property [maxBufferBindCount](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor/maxbufferbindcount) on the descriptor from which you created this argument table.

## Discussion

This method requires that the value of property [supportAttributeStrides](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor/supportattributestrides) on the descriptor from which you created this argument table is true.
