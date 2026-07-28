# setThreadgroupMemoryLength(_:index:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setthreadgroupmemorylength(_:index:)>

Configures the size of a threadgroup memory buffer for a threadgroup argument in the compute shader function.

## Declaration

```swift
func setThreadgroupMemoryLength(_ length: Int, index: Int)
```

## Parameters

- **length** — The size of the threadgroup memory, in bytes. Use a multiple of `16` bytes.
- **index** — An integer that corresponds to the index of the argument you annotate with attribute `[[threadgroup(index)]]` in the shader function.

## See also

### Configuring the pass
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setcomputepipelinestate(_:)) — Configures this encoder with a compute pipeline state that applies to your subsequent dispatch commands.
- [setArgumentTable(_:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setargumenttable(_:)) — Sets an argument table for the compute shader stage of this pipeline.
- [setImageblockSize(width:height:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setimageblocksize(width:height:)) — Specifies the size, in pixels, of imageblock data in tile memory.
