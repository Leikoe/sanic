# executeCommandsInBuffer(_:range:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 12.0, macOS 10.14, tvOS 12.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:range:)>

Encodes a command that runs a range of commands from an indirect command buffer (ICB).

## Declaration

```swift
func executeCommandsInBuffer(_ buffer: any MTLIndirectCommandBuffer, range: Range<Int>)
```

## Parameters

- **buffer** — An [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance that contains other commands the current command runs.
- **range** — A span of integers that represent the command entries in `buffer` the current command runs. When running on Metal devices that belong to the [MTLGPUFamily.mac2](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2) GPU family, the number of commands needs to be less than or equal to 0x4000 (16,384). Metal devices that belong to an Apple silicon family, such as [MTLGPUFamily.apple10](https://developer.apple.com/documentation/metal/mtlgpufamily/apple10), don’t have this limitation.

## See also

### Running commands from indirect command buffers
- [executeCommandsInBuffer(_:indirectBuffer:offset:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)) — Encodes a command that runs an indirect range of commands from an indirect command buffer (ICB).
