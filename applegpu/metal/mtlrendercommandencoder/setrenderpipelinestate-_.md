# setRenderPipelineState(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)>

Configures the encoder with a render or tile pipeline state that applies to your subsequent draw commands.

## Declaration

```swift
func setRenderPipelineState(_ pipelineState: any MTLRenderPipelineState)
```

## Parameters

- **pipelineState** — A render pipeline state that you create by calling an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods (see [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation)).

## Discussion

Set the render pass’s render pipeline state before encoding any draw or tile commands by calling this method because the default pipeline state is `nil`.

You can change which pipeline state the encoder uses multiple times during its lifetime. For example, your app may want render some things with a vertex shader, and render others with an object and mesh shader. Changing the pipeline state only affects the subsequent commands and has no effect on the commands you encode before changing the state.

The render pipeline you pass to this method needs to be compatible with the render pass’s attachments. You configure these attachments with the properties of an [MTLRenderPassDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor) instance, including [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/colorattachments), [depthAttachment](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/depthattachment), and [stencilAttachment](https://developer.apple.com/documentation/metal/mtlrenderpassdescriptor/stencilattachment).
