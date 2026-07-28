# setComputePipelineState(_:at:)

*Instance Method · iOS 13.0, iPadOS 13.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestate(_:at:)>

Encodes a reference to a compute pipeline state into the argument buffer.

## Declaration

```swift
func setComputePipelineState(_ pipeline: (any MTLComputePipelineState)?, at index: Int)
```

## Parameters

- **pipeline** — A pipeline state the method encodes.
- **index** — The index of a pipeline state within the argument buffer. The value corresponds to either the index ID of a declaration in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of an [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instance.

## See also

### Encoding pipeline states
- [setRenderPipelineState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setrenderpipelinestate(_:index:)) — Encodes a reference to a render pipeline state into the argument buffer.
- [setRenderPipelineStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setrenderpipelinestates(_:range:)) — Encodes references to an array of render pipeline states into the argument buffer.
- [setComputePipelineState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestate(_:index:)) — Encodes a reference to a compute pipeline state into the argument buffer.
- [setComputePipelineStates(_:with:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestates(_:with:)) — Encodes references to an array of compute pipeline states into the argument buffer.
- [setComputePipelineStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestates(_:range:)) — Encodes references to an array of compute pipeline states into the argument buffer.
