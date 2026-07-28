# setThreadgroupMemoryLength(_:at:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:at:)>

Sets the size of a block of threadgroup memory.

## Declaration

```swift
func setThreadgroupMemoryLength(_ length: Int, at index: Int)
```

## Parameters

- **length** — The size of the threadgroup memory, in bytes, which needs to be a multiple of 16 bytes.
- **index** — The index in the threadgroup memory argument table.

## See also

### Setting a command’s arguments
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setcomputepipelinestate(_:)) — Sets the command’s compute pipeline state.
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setimageblockwidth(_:height:)) — Sets the size, in pixels, of the imageblock.
- [setKernelBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setkernelbuffer(_:offset:at:)) — Sets a buffer for the compute function.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:index:)) — Sets the size of a block of threadgroup memory.
- [setStageInRegion(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstageinregion(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
- [setStageIn(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstagein(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
