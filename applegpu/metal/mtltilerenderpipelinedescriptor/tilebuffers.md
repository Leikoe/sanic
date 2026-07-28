# tileBuffers

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/tilebuffers>

An array that contains the buffer mutability options for a render pipeline’s tile function.

## Declaration

```swift
var tileBuffers: MTLPipelineBufferDescriptorArray { get }
```

## Discussion

This property returns an array of [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) objects, with each array index corresponding to the same index in the buffer argument table for the render pipeline’s tile shader.

## See also

### Specifying graphics functions and associated data
- [tileFunction](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/tilefunction) — The compute kernel or fragment function the pipeline calls.
- [maxCallStackDepth](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor/maxcallstackdepth) — The maximum call stack depth for indirect function calls in tile shaders.
