# executeCommandsInBuffer(_:indirectBuffer:offset:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.0, macOS 10.14, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:indirectbuffer:offset:)>

Encodes a command that runs an indirect range of commands from an indirect command buffer (ICB).

## Declaration

```swift
func executeCommandsInBuffer(_ buffer: any MTLIndirectCommandBuffer, indirectBuffer indirectRangeBuffer: any MTLBuffer, offset: Int)
```

## Parameters

- **buffer** — An [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance that contains other commands the current command runs.
- **indirectRangeBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) structure. When running on Metal devices that belong to the [MTLGPUFamily.mac2](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2) GPU family, the maximum value for the [length](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange/length) property of that structure is 0x4000 (16,384). Metal devices that belong to an Apple silicon family, such as [MTLGPUFamily.apple10](https://developer.apple.com/documentation/metal/mtlgpufamily/apple10), don’t have this limitation.
- **offset** — An integer that represents the location, in bytes, from the start of `indirectRangeBuffer` where the execution range structure begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.

## See also

### Running commands from indirect command buffers
- [executeCommandsInBuffer(_:range:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/executecommandsinbuffer(_:range:)) — Encodes a command that runs a range of commands from an indirect command buffer (ICB).
