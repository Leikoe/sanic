# makeRenderPipelineState(additionalBinaryFunctions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makerenderpipelinestate(additionalbinaryfunctions:)-49r1w>

Creates a new render pipeline state by adding binary functions to each stage of this pipeline state.

## Declaration

```swift
func makeRenderPipelineState(additionalBinaryFunctions binaryFunctionsDescriptor: MTL4RenderPipelineBinaryFunctionsDescriptor) throws -> any MTLRenderPipelineState
```

## Parameters

- **binaryFunctionsDescriptor** — A non-`nil` dynamic linking descriptor.

## Return Value

A new render pipeline state upon success, otherwise `nil`.
