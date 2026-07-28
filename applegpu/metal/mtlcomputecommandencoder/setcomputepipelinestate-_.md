# setComputePipelineState(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)>

Configures the compute encoder with a pipeline state for subsequent kernel calls.

## Declaration

```swift
func setComputePipelineState(_ state: any MTLComputePipelineState)
```

## Parameters

- **state** — An [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) instance.

## Discussion

> **Important:**
>  Set a compute encoder’s pipeline state before encoding any commands. Encoding commands without an available pipeline state causes an error.

Create your pipeline state through one of the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods in Creating Compute Pipeline States.

A compute pipeline state provides information Metal uses to compile and run encoded commands. You can change the pipeline state at any time, allowing you to encode multiple kernel calls in a single command buffer. Changing the pipeline state doesn’t affect any previously encoded commands.

## See also

### Configuring the pipeline state
- [dispatchType](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/dispatchtype) — The dispatch type to use when submitting compute work to the GPU.
