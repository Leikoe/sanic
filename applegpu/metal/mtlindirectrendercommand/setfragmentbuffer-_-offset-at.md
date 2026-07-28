# setFragmentBuffer(_:offset:at:)

*Instance Method · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setfragmentbuffer(_:offset:at:)>

Sets a fragment buffer argument for the command.

## Declaration

```swift
func setFragmentBuffer(_ buffer: any MTLBuffer, offset: Int, at index: Int)
```

## Parameters

- **buffer** — The buffer to set in the buffer argument table.
- **offset** — The location, in bytes relative to start of `buffer`, of the first byte of data for the fragment shader.
- **index** — An index in the buffer argument table. The maximum index is determined when you created the indirect command buffer.

## Discussion

You don’t need to call this method if you create an indirect command buffer with its [inheritBuffers](https://developer.apple.com/documentation/metal/mtlindirectcommandbufferdescriptor/inheritbuffers) equal to [true](https://developer.apple.com/documentation/Swift/true). The command gets the arguments from the parent encoder when it runs.

If you need to pass other kinds of parameters to your shader, such as textures and samplers, create an argument buffer and pass it to the shader using this method.

## See also

### Setting command arguments
- [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setrenderpipelinestate(_:)) — Sets the render pipeline state for the command.
- [setVertexBuffer(_:offset:at:)](https://developer.apple.com/documentation/metal/mtlindirectrendercommand/setvertexbuffer(_:offset:at:)) — Sets a vertex buffer argument for the command.
