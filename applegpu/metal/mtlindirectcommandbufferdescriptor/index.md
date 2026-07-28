# MTLIndirectCommandBufferDescriptor

*Class · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor>

A configuration you create to customize an indirect command buffer.

## Declaration

```swift
class MTLIndirectCommandBufferDescriptor
```

## Topics

### Declaring command types to encode
- [commandTypes](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/commandtypes) — The set of command types that you can encode into the indirect command buffer.

### Declaring command inheritance
- [inheritBuffers](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritbuffers) — A Boolean value that determines where commands in the indirect command buffer get their buffer arguments from when you execute them.
- [inheritPipelineState](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritpipelinestate) — A Boolean value that determines where commands in the indirect command buffer get their pipeline state from when you execute them.

### Declaring the maximum number of argument buffers per command
- [maxVertexBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxvertexbufferbindcount) — The maximum number of buffers that you can set per command for the vertex stage.
- [maxFragmentBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxfragmentbufferbindcount) — The maximum number of buffers that you can set per command for the fragment stage.
- [maxKernelBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxkernelbufferbindcount) — The maximum number of buffers that you can set per command for the compute kernel.

### Instance Properties
- [inheritCullMode](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritcullmode) — Configures whether the indirect command buffer inherits the cull mode from the encoder.
- [inheritDepthBias](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritdepthbias) — Configures whether the indirect command buffer inherits the depth bias from the encoder.
- [inheritDepthClipMode](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritdepthclipmode) — Configures whether the indirect command buffer inherits the depth clip mode from the encoder.
- [inheritDepthStencilState](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritdepthstencilstate) — Configures whether the indirect command buffer inherits the depth stencil state from the encoder.
- [inheritFrontFacingWinding](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritfrontfacingwinding) — Configures whether the indirect command buffer inherits the front facing winding from the encoder.
- [inheritTriangleFillMode](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inherittrianglefillmode) — Configures whether the indirect command buffer inherits the triangle fill mode from the encoder.
- [maxKernelThreadgroupMemoryBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxkernelthreadgroupmemorybindcount)
- [maxMeshBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxmeshbufferbindcount)
- [maxObjectBufferBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxobjectbufferbindcount)
- [maxObjectThreadgroupMemoryBindCount](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/maxobjectthreadgroupmemorybindcount)
- [supportColorAttachmentMapping](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/supportcolorattachmentmapping) — Specifies if the indirect command buffer should support color attachment mapping.
- [supportDynamicAttributeStride](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/supportdynamicattributestride)
- [supportRayTracing](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/supportraytracing)

## See also

### Indirect command buffers
- [Creating an indirect command buffer](https://developer.apple.com/documentation/metal/creating-an-indirect-command-buffer) — Configure a descriptor to specify the properties of an indirect command buffer.
- [Specifying drawing and dispatch arguments indirectly](https://developer.apple.com/documentation/metal/specifying-drawing-and-dispatch-arguments-indirectly) — Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
- [Encoding indirect command buffers on the CPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-cpu) — Reduce CPU overhead and simplify your command execution by reusing commands.
- [Encoding indirect command buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu) — Maximize CPU to GPU parallelization by generating render commands on the GPU.
- [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer) — A command buffer containing reusable commands, encoded either on the CPU or GPU.
- [MTLIndirectCommandType](https://developer.apple.com/documentation/metal/mtlindirectcommandtype) — The types of commands that you can encode into the indirect command buffer.
- [MTLIndirectCommandBufferExecutionRange](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrange) — A range of commands in an indirect command buffer.
- [MTLIndirectCommandBufferExecutionRangeMake(_:_:)](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferexecutionrangemake(_:_:)) — Creates a command execution range.
