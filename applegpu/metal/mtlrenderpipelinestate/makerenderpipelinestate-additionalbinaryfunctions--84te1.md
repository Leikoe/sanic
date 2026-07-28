# makeRenderPipelineState(additionalBinaryFunctions:)

*Instance Method · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makerenderpipelinestate(additionalbinaryfunctions:)-84te1>

Creates a new pipeline state that’s a copy of the current pipeline state with additional shaders.

## Declaration

```swift
func makeRenderPipelineState(additionalBinaryFunctions: MTLRenderPipelineFunctionsDescriptor) throws -> any MTLRenderPipelineState
```

## Parameters

- **additionalBinaryFunctions** — An [MTLRenderPipelineFunctionsDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinefunctionsdescriptor) instance, which contains [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) arrays for vertex, fragment, and tile shaders.
