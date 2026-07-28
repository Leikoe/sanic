# executeCommands(in:indirectBuffer:indirectBufferOffset:)

*Instance Method · iOS 13.0, iPadOS 13.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:indirectbuffer:indirectbufferoffset:)>

Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.

## Declaration

```swift
func executeCommands(in indirectCommandbuffer: any MTLIndirectCommandBuffer, indirectBuffer indirectRangeBuffer: any MTLBuffer, indirectBufferOffset: Int)
```

## Parameters

- **indirectCommandbuffer** — The [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance containing the commands to execute.
- **indirectRangeBuffer** — An indirect buffer containing the execution range, laid out in an [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) instance. The maximum length of the range is `16384` commands.
- **indirectBufferOffset** — The number of bytes from the start of `indirectRangeBuffer` containing the execution range to use. Align the offset on a multiple of `4`.

## See also

### Dispatching from indirect command buffers
- [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) — Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)) — Encodes an instruction to run commands from an indirect buffer.
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:with:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:with:)) — Encodes an instruction to run commands from an indirect buffer.
