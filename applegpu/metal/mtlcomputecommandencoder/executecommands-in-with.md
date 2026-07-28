# executeCommands(in:with:)

*Instance Method · iOS 13.0, iPadOS 13.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:with:)>

Encodes an instruction to run commands from an indirect buffer.

## Declaration

```swift
func executeCommands(in indirectCommandBuffer: any MTLIndirectCommandBuffer, with executionRange: NSRange)
```

## Parameters

- **indirectCommandBuffer** — The [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance containing the commands to execute.
- **executionRange** — The range of commands to execute. The maximum length of the range is `16384` commands.

## See also

### Dispatching from indirect command buffers
- [dispatchThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchthreadgroups(indirectbuffer:indirectbufferoffset:threadsperthreadgroup:)) — Encodes a dispatch call for a compute pass, using an indirect buffer that defines the size of a grid that aligns to threadgroup boundaries.
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:range:)) — Encodes an instruction to run commands from an indirect buffer.
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
- [executeCommands(in:indirectBuffer:indirectBufferOffset:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/executecommands(in:indirectbuffer:indirectbufferoffset:)) — Encodes an instruction to run commands from an indirect buffer, using another buffer to provide the command range.
