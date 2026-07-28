# functionHandle(function:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/functionhandle(function:)-w9ia>

Get the function handle for the specified binary-linked function from the pipeline state.

## Declaration

```swift
func functionHandle(function: any MTL4BinaryFunction) -> (any MTLFunctionHandle)?
```

## Parameters

- **function** — A [MTL4BinaryFunction](https://developer.apple.com/documentation/metal/mtl4binaryfunction) instance representing the function binary.

## Return Value

A [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) instance  for a binary function that was compiled with `MTLFunctionOptionPipelineIndependent`, otherwise `nil`.
