# Command buffer debugging

*API Collection*

<https://developer.apple.com/documentation/metal/command-buffer-debugging>

Properties and methods for programmatically debugging runtime issues with a command buffer.

## Topics

### Identifying the command buffer
- [label](https://developer.apple.com/documentation/metal/mtlcommandbuffer/label) — An optional name that can help you identify the command buffer.
- [commandQueue](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commandqueue) — The command queue that creates the command buffer.
- [device](https://developer.apple.com/documentation/metal/mtlcommandbuffer/device) — The GPU device that indirectly owns the command buffer because you create it from a command queue the device also owns.

### Grouping commands within a GPU frame capture
- [pushDebugGroup(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/pushdebuggroup(_:)) — Marks the beginning of a debug group and gives it an identifying label, which temporarily replaces the previous group, if applicable.
- [popDebugGroup()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/popdebuggroup()) — Marks the end of a debug group and, if applicable, restores the previous group from a stack.

### Getting error details
- [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) — A description of an error when the GPU encounters an issue as it runs the command buffer.
- [errorOptions](https://developer.apple.com/documentation/metal/mtlcommandbuffer/erroroptions) — Settings that determine which information the command buffer records about execution errors, and how it does it.
- [MTLCommandBufferEncoderInfo](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfo) — A container that provides additional information about a runtime failure a GPU encounters as it runs the commands in a command buffer.
- [MTLCommandBufferEncoderInfoErrorKey](https://developer.apple.com/documentation/metal/mtlcommandbufferencoderinfoerrorkey) — A key to a command buffer error’s user information dictionary that retrieves additional information about a GPU’s runtime error.

### Reading the runtime message logs
- [logs](https://developer.apple.com/documentation/metal/mtlcommandbuffer/logs-518l2) — The messages the command buffer records as the GPU runs its commands.

### Checking scheduling times on the CPU
- [kernelStartTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelstarttime) — The host time, in seconds, when the CPU begins to schedule the command buffer.
- [kernelEndTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelendtime) — The host time, in seconds, when the CPU finishes scheduling the command buffer.

### Checking execution times on the GPU
- [gpuStartTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpustarttime) — The host time, in seconds, when the GPU starts command buffer execution.
- [gpuEndTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpuendtime) — The host time, in seconds, when the GPU finishes execution of the command buffer.

### Determining whether to maintain strong references
- [retainedReferences](https://developer.apple.com/documentation/metal/mtlcommandbuffer/retainedreferences) — A Boolean value that indicates whether the command buffer maintains strong references to the resources it uses.

## See also

### Troubleshooting a command buffer
- [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) — The command buffer’s current state.
- [MTLCommandBufferStatus](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus) — The discrete states for a command buffer that represent its life cycle stages.
