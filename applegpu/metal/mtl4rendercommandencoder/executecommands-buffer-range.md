# executeCommands(buffer:range:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:range:)>

Encodes a command that runs a range of commands from an indirect command buffer.

## Declaration

```swift
func executeCommands(buffer: any MTLIndirectCommandBuffer, range: Range<Int>)
```

## Parameters

- **buffer** — An [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance that contains other commands the current command runs.
- **range** — A span of integers that represent the command entries in buffer the current command runs.

## See also

### Running commands from indirect command buffers
- [executeCommands(buffer:indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:indirectbuffer:)) — Encodes a command that runs an indirect range of commands from an indirect command buffer.
