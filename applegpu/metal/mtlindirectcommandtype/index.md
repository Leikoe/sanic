# MTLIndirectCommandType

*Structure · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandtype>

The types of commands that you can encode into the indirect command buffer.

## Declaration

```swift
struct MTLIndirectCommandType
```

## Topics

### Creating a set of command types
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/init(rawvalue:)) — Initializes the set of command types from a raw integer value.

### Specifying command types
- [draw](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/draw) — A draw call command.
- [drawIndexed](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/drawindexed) — An indexed draw call command.
- [drawPatches](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/drawpatches) — A draw call command for tessellated patches.
- [drawIndexedPatches](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/drawindexedpatches) — An indexed draw call command for tessellated patches.
- [concurrentDispatch](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/concurrentdispatch) — A compute command using a grid aligned to threadgroup boundaries.
- [concurrentDispatchThreads](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/concurrentdispatchthreads) — A compute command using an arbitrarily sized grid.

### Type Properties
- [drawMeshThreadgroups](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/drawmeshthreadgroups)
- [drawMeshThreads](https://developer.apple.com/documentation/metal/mtlindirectcommandtype/drawmeshthreads)

## See also

### Indirect command buffers
- [Creating an indirect command buffer](https://developer.apple.com/documentation/metal/creating-an-indirect-command-buffer) — Configure a descriptor to specify the properties of an indirect command buffer.
- [Specifying drawing and dispatch arguments indirectly](https://developer.apple.com/documentation/metal/specifying-drawing-and-dispatch-arguments-indirectly) — Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Encoding indirect command buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu) — Maximize CPU to GPU parallelization by generating render commands on the GPU.
- [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) — A command buffer containing reusable commands, encoded either on the CPU or GPU.
- [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) — A configuration you create to customize an indirect command buffer.
- [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) — A range of commands in an indirect command buffer.
- [MTLIndirectCommandBufferExecutionRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrangemake(_:_:)) — Creates a command execution range.
