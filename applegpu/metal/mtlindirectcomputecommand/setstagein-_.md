# setStageIn(_:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstagein(_:)>

Sets the region of the stage-in attributes to apply to the compute kernel.

## Declaration

```swift
func setStageIn(_ region: MTLRegion)
```

## Parameters

- **region** — The offset and maximum size of the grid over which compute threads that read per-thread stage-in data are launched.

## See also

### Setting a command’s arguments
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setcomputepipelinestate(_:)) — Sets the command’s compute pipeline state.
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setimageblockwidth(_:height:)) — Sets the size, in pixels, of the imageblock.
- [setKernelBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setkernelbuffer(_:offset:at:)) — Sets a buffer for the compute function.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:index:)) — Sets the size of a block of threadgroup memory.
- [setThreadgroupMemoryLength(_:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:at:)) — Sets the size of a block of threadgroup memory.
- [setStageInRegion(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstageinregion(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
