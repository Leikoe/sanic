# executeCommands(buffer:indirectBuffer:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:indirectbuffer:)>

Encodes an instruction to execute commands from an indirect command buffer, using an indirect buffer for arguments.

## Declaration

```swift
func executeCommands(buffer indirectCommandbuffer: any MTLIndirectCommandBuffer, indirectBuffer indirectRangeBuffer: MTLGPUAddress)
```

## Parameters

- **indirectCommandbuffer** — [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance containing the commands to execute.
- **indirectRangeBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) containing the execution range. Lay out the data in this buffer as described in the [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) structure. This address requires 4-byte alignment.

## Discussion

Use this method to indicate to Metal the span of indices in the command buffer to execute indirectly via an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance you provide in the `indirectRangeBuffer` parameter. This allows you to calculate the span of commands Metal executes in the GPU timeline, enabling GPU-driven workflows.

Metal requires that the contents of this buffer match the layout of struct [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange), which specifies a location and a length within the indirect command buffer. You are responsible for ensuring the address of this buffer has 4-byte alignment.

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the indirect buffer that the `indirectRangeBuffer` parameter references.

> **Note:**
> If the `indirectCommandBuffer` parameter references any pipeline state objects, you are responsible for adding them to a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) instance in use when you commit the command buffer.
> 
> An indirect compute command references a pipeline state when you pass it as an argument to the command’s [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectcomputecommand/setcomputepipelinestate(_:)) method during CPU encoding, or `set_compute_pipeline_state()` during GPU encoding.

## See also

### Encoding indirect command buffers
- [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/executecommands(buffer:range:)) — Encodes a command to execute commands from an indirect command buffer.
