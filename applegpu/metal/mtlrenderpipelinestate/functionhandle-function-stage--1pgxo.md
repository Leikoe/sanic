# functionHandle(function:stage:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/functionhandle(function:stage:)-1pgxo>

Obtains the function handle for a specific function this pipeline state links at the binary level.

## Declaration

```swift
func functionHandle(function: any MTL4BinaryFunction, stage: MTLRenderStages) -> (any MTLFunctionHandle)?
```

## Parameters

- **function** — A binary function to retrieve the handle.
- **stage** — The shader stage that uses the function.

## Return Value

A function handle representing the function if present, otherwise `nil`.
