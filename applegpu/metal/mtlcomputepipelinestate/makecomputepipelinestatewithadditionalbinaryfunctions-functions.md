# makeComputePipelineStateWithAdditionalBinaryFunctions(functions:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makecomputepipelinestatewithadditionalbinaryfunctions(functions:)>

Creates a new pipeline state object with additional callable functions.

## Declaration

```swift
func makeComputePipelineStateWithAdditionalBinaryFunctions(functions: [any MTLFunction]) throws -> any MTLComputePipelineState
```

## Parameters

- **functions** — The list of additional functions that you want to be able to call.

## Return Value

A new compute pipeline state with access to the provided functions. When this value is `nil`, an error occurred during handle creation.
