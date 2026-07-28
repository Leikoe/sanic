# setArgumentTable(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setargumenttable(_:)>

Sets an argument table for the compute shader stage of this pipeline.

## Declaration

```swift
func setArgumentTable(_ argumentTable: (any MTL4ArgumentTable)?)
```

## Parameters

- **argumentTable** — A [MTL4ArgumentTable](https://developer.apple.com/documentation/metal/mtl4argumenttable) to set on the command encoder.

## Discussion

Metal takes a snapshot of the resources in the argument table when you make dispatch or execute calls on this encoder instance. Metal makes the snapshot contents available to the compute shader function of the current pipeline state.

## See also

### Configuring the pass
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setcomputepipelinestate(_:)) — Configures this encoder with a compute pipeline state that applies to your subsequent dispatch commands.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setthreadgroupmemorylength(_:index:)) — Configures the size of a threadgroup memory buffer for a threadgroup argument in the compute shader function.
- [setImageblockSize(width:height:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/setimageblocksize(width:height:)) — Specifies the size, in pixels, of imageblock data in tile memory.
