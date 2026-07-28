# setThreadgroupMemoryLength(_:index:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setthreadgroupmemorylength(_:index:)>

Configures the size of a block of threadgroup memory.

## Declaration

```swift
func setThreadgroupMemoryLength(_ length: Int, index: Int)
```

## Parameters

- **length** — The size of the threadgroup memory, in bytes, which needs to be a multiple of `16` bytes.
- **index** — The index in the threadgroup memory argument table using this allocation.

## Discussion

> **Important:**
>  The sum of all threadgroup memory allocations (whether made using this method or directly in the shader) can’t exceed the device limits for threadgroup memory. Check threadgroup memory limits with the [staticThreadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/staticthreadgroupmemorylength) property.

The `threadgroup` memory space allows for sharing data between multiple threads in a threadgroup, which can be faster than using `device` memory in your kernels. Before using any threadgroup memory, call this method to configure the threadgroup memory argument table. Kernels accessing their arguments from threadgroup memory have the `[[threadgroup]]` attribute.

To learn more about using the threadgroup address space, see the [Metal Shading Language Specification](https://developer.apple.com/metal/metal-shading-language-specification.pdf#//apple_ref/doc/uid/TP40014364-CH4-SW5) section 4.4.

## See also

### Configuring tile memory
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setimageblockwidth(_:height:)) — Sets the size, in pixels, of imageblock data in tile memory.
