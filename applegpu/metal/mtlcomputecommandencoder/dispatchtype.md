# dispatchType

*Instance Property · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchtype>

The dispatch type to use when submitting compute work to the GPU.

## Declaration

```swift
var dispatchType: MTLDispatchType { get }
```

## Discussion

You set this property when you create the command encoder, and it doesn’t change for the remainder of the encoding.

See [makeComputeCommandEncoder(dispatchType:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/makecomputecommandencoder(dispatchtype:)) for more information.

## See also

### Configuring the pipeline state
- [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)) — Configures the compute encoder with a pipeline state for subsequent kernel calls.
