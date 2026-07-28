# setComputePipelineState(_:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setcomputepipelinestate(_:)>

Sets the command’s compute pipeline state.

## Declaration

```swift
func setComputePipelineState(_ pipelineState: any MTLComputePipelineState)
```

## Parameters

- **pipelineState** — A compute pipeline state instance.

## Discussion

You don’t need to call this method if you create an indirect command buffer with its [inheritPipelineState](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritpipelinestate) property equal to [true](https://developer.apple.com/documentation/Swift/true). The command gets the pipeline state from the parent encoder when you run the command.

If you create an indirect command buffer with its [inheritPipelineState](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritpipelinestate) property equal to [false](https://developer.apple.com/documentation/Swift/false), you need to set the pipeline state prior to encoding a drawing command.

## See also

### Setting a command’s arguments
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setimageblockwidth(_:height:)) — Sets the size, in pixels, of the imageblock.
- [setKernelBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setkernelbuffer(_:offset:at:)) — Sets a buffer for the compute function.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:index:)) — Sets the size of a block of threadgroup memory.
- [setThreadgroupMemoryLength(_:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:at:)) — Sets the size of a block of threadgroup memory.
- [setStageInRegion(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstageinregion(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
- [setStageIn(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstagein(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
