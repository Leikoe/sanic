# setKernelBuffer(_:offset:at:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 11.0, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setkernelbuffer(_:offset:at:)>

Sets a buffer for the compute function.

## Declaration

```swift
func setKernelBuffer(_ buffer: any MTLBuffer, offset: Int, at index: Int)
```

## Parameters

- **buffer** — The buffer to set in the buffer argument table.
- **offset** — Where the data begins, in bytes, from the start of the buffer.
- **index** — An index in the buffer argument table.

## Discussion

If you created the indirect command buffer with [inheritBuffers](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritbuffers) set to [true](https://developer.apple.com/documentation/Swift/true), don’t call this method. The command gets the arguments from the parent encoder when you execute the command.

If you need to pass other kinds of parameters to your shader, such as textures and samplers, create an argument buffer and pass it to the shader using this method.

## See also

### Setting a command’s arguments
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setcomputepipelinestate(_:)) — Sets the command’s compute pipeline state.
- [setImageblockWidth(_:height:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setimageblockwidth(_:height:)) — Sets the size, in pixels, of the imageblock.
- [setThreadgroupMemoryLength(_:index:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:index:)) — Sets the size of a block of threadgroup memory.
- [setThreadgroupMemoryLength(_:at:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setthreadgroupmemorylength(_:at:)) — Sets the size of a block of threadgroup memory.
- [setStageInRegion(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstageinregion(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
- [setStageIn(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setstagein(_:)) — Sets the region of the stage-in attributes to apply to the compute kernel.
