# executeCommands(buffer:indirectBuffer:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:indirectbuffer:)>

Encodes a command that runs an indirect range of commands from an indirect command buffer.

## Declaration

```swift
func executeCommands(buffer indirectCommandBuffer: any MTLIndirectCommandBuffer, indirectBuffer indirectRangeBuffer: MTLGPUAddress)
```

## Parameters

- **indirectCommandBuffer** — A [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) instance that contains other commands the current command runs.
- **indirectRangeBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) structure. This address requires 4-byte alignment.

## Discussion

Use this method to indicate to Metal the span of indices in the command buffer to execute indirectly via an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance you provide in the `indirectRangeBuffer` parameter. This allows you to calculate the span of commands Metal executes in the GPU timeline, enabling GPU-driven workflows.

Metal requires that the contents of this buffer match the layout of struct [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange), which specifies a location and a length within the indirect command buffer. You are responsible for ensuring the address of this buffer has 4-byte alignment.

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the indirect buffer that the `indirectRangeBuffer` parameter references.

> **Note:**
> If the `indirectCommandBuffer` parameter references any pipeline state objects, you are responsible for adding them to a [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) instance in use when you commit the command buffer.
> 
> An indirect render command references a pipeline state when you pass it as an argument to the command’s [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setrenderpipelinestate(_:)) method during CPU encoding, or `set_render_pipeline_state()` during GPU encoding.

## See also

### Running commands from indirect command buffers
- [executeCommands(buffer:range:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/executecommands(buffer:range:)) — Encodes a command that runs a range of commands from an indirect command buffer.
