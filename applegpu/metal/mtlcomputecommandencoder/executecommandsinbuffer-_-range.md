# executeCommandsInBuffer(_:range:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 14.0, macOS 11.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)>

Encodes an instruction to run commands from an indirect buffer.

## Declaration

```swift
func executeCommandsInBuffer(_ buffer: any MTLIndirectCommandBuffer, range: Range<Int>)
```

## Parameters

- **buffer** — The [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance containing the commands to execute.
- **range** — The range of commands to execute. When running on Metal devices that belong to the [MTLGPUFamily.mac2](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2) GPU family, the maximum length of the range is 0x4000 (16,384) commands. Metal devices that belong to an Apple silicon family, such as [MTLGPUFamily.apple10](https://developer.apple.com/documentation/metal/mtlgpufamily/apple10), don’t have this limitation.

## See also

### Dispatching from indirect command buffers
- [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) — Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:indirectbuffer:indirectbufferoffset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:with:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:with:)) — Encodes an instruction to run commands from an indirect buffer.
