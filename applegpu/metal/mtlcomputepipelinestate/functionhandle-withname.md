# functionHandle(withName:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/functionhandle(withname:)>

Gets the function handle for a function this pipeline links at the Metal IR level by name.

## Declaration

```swift
func functionHandle(withName name: String) -> (any MTLFunctionHandle)?
```

## Parameters

- **name** — A string representing the name of the function.

## Return Value

A function handle corresponding to the function if the name matches a function in this pipeline state, otherwise `nil`.
