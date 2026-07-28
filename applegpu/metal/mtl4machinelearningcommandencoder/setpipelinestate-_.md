# setPipelineState(_:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/setpipelinestate(_:)>

Configures the encoder with a machine learning pipeline state instance.

## Declaration

```swift
func setPipelineState(_ pipelineState: any MTL4MachineLearningPipelineState)
```

## Parameters

- **pipelineState** — A Machine Learning pipeline state instance.

## Discussion

The pipeline state instance affects all subsequent Machine Learning commands.

## See also

### Configuring the pass
- [setArgumentTable(_:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/setargumenttable(_:)) — Sets an argument table for the command encoder’s machine learning shader stage.
