# MTL4MachineLearningPipelineState

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate>

A pipeline state that you can use with machine-learning encoder instances.

## Declaration

```swift
protocol MTL4MachineLearningPipelineState : MTLAllocation, Sendable
```

## Overview

See [MTL4MachineLearningCommandEncoder](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder) for more information.

## Topics

### Instance Properties
- [device](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate/device) — Returns the device the pipeline state belongs to.
- [intermediatesHeapSize](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate/intermediatesheapsize) — Obtain the size of the heap, in bytes, this pipeline requires during the execution.
- [label](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate/label) — Queries the string that helps identify this object.
- [reflection](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate/reflection) — Returns reflection information for this machine learning pipeline state.

## See also

### Encoding a machine learning pass
- [Running a machine learning model on the GPU timeline](https://developer.apple.com/documentation/metal/running-a-machine-learning-model-on-the-gpu-timeline) — Dispatch model inference commands with a machine learning pass in a Metal 4 command buffer.
- [MTL4MachineLearningCommandEncoder](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder) — Encodes machine learning model inference commands for a single pass.
