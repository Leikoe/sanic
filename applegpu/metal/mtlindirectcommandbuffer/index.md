# MTLIndirectCommandBuffer

*Protocol · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer>

A command buffer containing reusable commands, encoded either on the CPU or GPU.

## Declaration

```swift
protocol MTLIndirectCommandBuffer : MTLResource
```

## Overview

Use an indirect command buffer to encode commands once and reuse them, and to encode commands on multiple CPU or GPU threads.

Don’t implement this protocol yourself; instead, create an [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) instance, configure its properties, and tell the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) to create the indirect command buffer. See [Creating an indirect command buffer](https://developer.apple.com/documentation/metal/creating-an-indirect-command-buffer).

## Topics

### Determining the maximum number of commands
- [size](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/size) — The number of commands contained in the indirect command buffer.

### Retrieving commands
- [indirectRenderCommandAt(_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectrendercommandat(_:)) — Gets the render command at the given index.
- [indirectComputeCommandAt(_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectcomputecommandat(_:)) — Gets the compute command at the given index.
- [indirectComputeCommand(at:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/indirectcomputecommand(at:)) — Gets the compute command at the given index.

### Resetting commands
- [reset(_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/reset(_:)) — Resets a range of commands to their default state.

### Instance Properties
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer/gpuresourceid)

## See also

### Indirect command buffers
- [Creating an indirect command buffer](https://developer.apple.com/documentation/metal/creating-an-indirect-command-buffer) — Configure a descriptor to specify the properties of an indirect command buffer.
- [Specifying drawing and dispatch arguments indirectly](https://developer.apple.com/documentation/metal/specifying-drawing-and-dispatch-arguments-indirectly) — Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Encoding indirect command buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu) — Maximize CPU to GPU parallelization by generating render commands on the GPU.
- [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) — A configuration you create to customize an indirect command buffer.
- [MTLIndirectCommandType](https://developer.apple.com/documentation/metal/mtlindirectcommandtype) — The types of commands that you can encode into the indirect command buffer.
- [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) — A range of commands in an indirect command buffer.
- [MTLIndirectCommandBufferExecutionRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrangemake(_:_:)) — Creates a command execution range.
