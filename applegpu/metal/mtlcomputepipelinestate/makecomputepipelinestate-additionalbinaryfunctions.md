# makeComputePipelineState(additionalBinaryFunctions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makecomputepipelinestate(additionalbinaryfunctions:)>

Allocates a new compute pipeline state by adding binary functions to this pipeline state.

## Declaration

```swift
func makeComputePipelineState(additionalBinaryFunctions: [any MTL4BinaryFunction]) throws -> any MTLComputePipelineState
```

## Parameters

- **additionalBinaryFunctions** — A non-`nil` array containing binary functions to add to this pipeline.

## Return Value

A new compute pipeline state upon success, otherwise `nil`.
