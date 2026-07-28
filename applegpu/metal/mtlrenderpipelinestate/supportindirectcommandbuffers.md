# supportIndirectCommandBuffers

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/supportindirectcommandbuffers>

A Boolean value that indicates whether the render pipeline supports encoding commands into an indirect command buffer.

## Declaration

```swift
var supportIndirectCommandBuffers: Bool { get }
```

## Discussion

This property gets its value by copying from the [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/supportindirectcommandbuffers) property of the [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) instance as the GPU device creates the pipeline state.
