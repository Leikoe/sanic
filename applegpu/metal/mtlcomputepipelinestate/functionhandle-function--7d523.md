# functionHandle(function:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/functionhandle(function:)-7d523>

Creates a function handle for a visible function.

## Declaration

```swift
func functionHandle(function: any MTLFunction) -> (any MTLFunctionHandle)?
```

## Parameters

- **function** — An [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance that represents the visible function to create a handle for.

## Return Value

A handle to the visible function. When this value is `nil`, an error occurred during handle creation.
