# setComputePipelineStates(_:with:)

*Instance Method · iOS 13.0, iPadOS 13.0, tvOS 13.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestates(_:with:)>

Encodes references to an array of compute pipeline states into the argument buffer.

## Declaration

```swift
func setComputePipelineStates(_ pipelines: UnsafePointer<(any MTLComputePipelineState)?>, with range: NSRange)
```

## Parameters

- **pipelines** — An array of pipeline states the method encodes.
- **range** — A range of indices within the argument buffer for each element in `pipelines`. The values correspond to either the index IDs of declarations in Metal Shading Language (MSL) or the [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) property of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances.

## See also

### Encoding pipeline states
- [setRenderPipelineState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setrenderpipelinestate(_:index:)) — Encodes a reference to a render pipeline state into the argument buffer.
- [setRenderPipelineStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setrenderpipelinestates(_:range:)) — Encodes references to an array of render pipeline states into the argument buffer.
- [setComputePipelineState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestate(_:index:)) — Encodes a reference to a compute pipeline state into the argument buffer.
- [setComputePipelineState(_:at:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestate(_:at:)) — Encodes a reference to a compute pipeline state into the argument buffer.
- [setComputePipelineStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestates(_:range:)) — Encodes references to an array of compute pipeline states into the argument buffer.
