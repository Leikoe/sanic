# Creating an indirect command buffer

*Article*

<https://developer.apple.com/documentation/metal/creating-an-indirect-command-buffer>

Configure a descriptor to specify the properties of an indirect command buffer.

## Overview

An indirect command buffer stores encoded GPU commands persistently. Using an indirect command buffer, you can encode a command once and reuse it multiple times. You can also encode commands into an indirect command buffer simultaneously with multiple threads on the CPU or with a compute kernel on the GPU.

To create an indirect command buffer, first create an [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) instance and configure the descriptor’s properties. Then call [makeIndirectCommandBuffer(descriptor:maxCommandCount:options:)](https://developer.apple.com/documentation/metal/mtldevice/makeindirectcommandbuffer(descriptor:maxcommandcount:options:)) on an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance to create the indirect command buffer.

## See also

### Indirect command buffers
- [Specifying drawing and dispatch arguments indirectly](https://developer.apple.com/documentation/metal/specifying-drawing-and-dispatch-arguments-indirectly) — Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Encoding indirect command buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu) — Maximize CPU to GPU parallelization by generating render commands on the GPU.
- [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) — A command buffer containing reusable commands, encoded either on the CPU or GPU.
- [MTLIndirectCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor) — A configuration you create to customize an indirect command buffer.
- [MTLIndirectCommandType](https://developer.apple.com/documentation/metal/mtlindirectcommandtype) — The types of commands that you can encode into the indirect command buffer.
- [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) — A range of commands in an indirect command buffer.
- [MTLIndirectCommandBufferExecutionRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrangemake(_:_:)) — Creates a command execution range.
