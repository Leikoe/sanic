# executeCommandsInBuffer(_:indirectBuffer:offset:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)>

Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.

## Declaration

```swift
func executeCommandsInBuffer(_ buffer: any MTLIndirectCommandBuffer, indirectBuffer indirectRangeBuffer: any MTLBuffer, offset: Int)
```

## Parameters

- **buffer** — The [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance containing the commands to execute.
- **indirectRangeBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) structure. When running on Metal devices that belong to the [MTLGPUFamily.mac2](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2) GPU family, the maximum value for the [length](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange/length) property of that structure is 0x4000 (16,384). Metal devices that belong to an Apple silicon family, such as [MTLGPUFamily.apple10](https://developer.apple.com/documentation/metal/mtlgpufamily/apple10), don’t have this limitation.
- **offset** — The number of bytes from the start of `indirectRangeBuffer` containing the execution range to use. Align the offset on a multiple of `4`.

## See also

### Dispatching from indirect command buffers
- [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) — Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)) — Encodes an instruction to run commands from an indirect buffer.
- [executeCommands(in:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:indirectbuffer:indirectbufferoffset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:with:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:with:)) — Encodes an instruction to run commands from an indirect buffer.
