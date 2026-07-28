# functionHandle(function:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/functionhandle(function:)-8spaa>

Gets the function handle for a function this pipeline links at the binary level.

## Declaration

```swift
func functionHandle(function: any MTL4BinaryFunction) -> (any MTLFunctionHandle)?
```

## Parameters

- **function** — A binary function object representing the function binary to find.

## Return Value

A function handle corresponding to the function if the binary function matches a function in this pipeline state, otherwise `nil`.
