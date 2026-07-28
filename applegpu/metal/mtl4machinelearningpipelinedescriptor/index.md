# MTL4MachineLearningPipelineDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor>

Description for a machine learning pipeline state.

## Declaration

```swift
class MTL4MachineLearningPipelineDescriptor
```

## Topics

### Instance Properties
- [label](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/label) — Assigns an optional string that helps identify pipeline states you create from this descriptor.
- [machineLearningFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/machinelearningfunctiondescriptor) — Assigns the function that the machine learning pipeline you create from this descriptor executes.

### Instance Methods
- [inputDimensions(bufferIndex:)](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/inputdimensions(bufferindex:)) — Obtains the dimensions of the input tensor at `bufferIndex` if set, `nil` otherwise.
- [reset()](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/reset()) — Resets the descriptor to its default values.
- [setInputDimensions(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/setinputdimensions(_:bufferindex:)-34gir) — Sets the dimension of an input tensor at a buffer index.
- [setInputDimensions(_:bufferIndex:)](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinedescriptor/setinputdimensions(_:bufferindex:)-8fnq7) — Sets the dimensions of multiple input tensors on a range of buffer bindings.

## See also

### Configuring a machine learning pipeline
- [MTL4MachineLearningPipelineReflection](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinereflection) — Represents reflection information for a machine learning pipeline state.
