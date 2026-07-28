# MTLIndirectCommandBufferExecutionRange

*Structure · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.0, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange>

A range of commands in an indirect command buffer.

## Declaration

```swift
struct MTLIndirectCommandBufferExecutionRange
```

## Topics

### Creating a command execution range
- [init()](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange/init()) — Initializes an empty command execution range.
- [init(location:length:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange/init(location:length:)) — Initializes an command execution range.
- [MTLIndirectCommandBufferExecutionRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrangemake(_:_:)) — Creates a command execution range.

### Examining the range
- [location](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange/location) — The first index in the command execution range.
- [length](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange/length) — The number of items in the command execution range.

## See also

### Indirect command buffers
- [Creating an indirect command buffer](https://developer.apple.com/documentation/metal/creating-an-indirect-command-buffer) — Configure a descriptor to specify the properties of an indirect command buffer.
- [Specifying drawing and dispatch arguments indirectly](https://developer.apple.com/documentation/metal/specifying-drawing-and-dispatch-arguments-indirectly) — Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Encoding indirect command buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu) — Maximize CPU to GPU parallelization by generating render commands on the GPU.
- [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) — A command buffer containing reusable commands, encoded either on the CPU or GPU.
- [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) — A configuration you create to customize an indirect command buffer.
- [MTLIndirectCommandType](https://developer.apple.com/documentation/metal/mtlindirectcommandtype) — The types of commands that you can encode into the indirect command buffer.
- [MTLIndirectCommandBufferExecutionRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrangemake(_:_:)) — Creates a command execution range.
