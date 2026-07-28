# setRenderPipelineState(_:)

*Instance Method · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.0, macOS 10.14, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setrenderpipelinestate(_:)>

Sets the render pipeline state for the command.

## Declaration

```swift
func setRenderPipelineState(_ pipelineState: any MTLRenderPipelineState)
```

## Parameters

- **pipelineState** — The rendering pipeline state object to use.

## Discussion

You don’t need to call this method if you create an indirect command buffer with its [inheritPipelineState](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritpipelinestate) property equal to [true](https://developer.apple.com/documentation/Swift/true). The command gets the pipeline state from the parent encoder when it runs.

If you created the indirect command buffer with [inheritPipelineState](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritpipelinestate) set to [false](https://developer.apple.com/documentation/Swift/false), you need to set the pipeline state prior to encoding the drawing command.

## See also

### Setting command arguments
- [setVertexBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setvertexbuffer(_:offset:at:)) — Sets a vertex buffer argument for the command.
- [setFragmentBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setfragmentbuffer(_:offset:at:)) — Sets a fragment buffer argument for the command.
