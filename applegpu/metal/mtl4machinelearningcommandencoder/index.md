# MTL4MachineLearningCommandEncoder

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder>

Encodes machine learning model inference commands for a single pass.

## Declaration

```swift
protocol MTL4MachineLearningCommandEncoder : MTL4CommandEncoder
```

## Overview

Create a machine learning encoder by calling a factory method of an [MTL4CommandBuffer](https://developer.apple.com/documentation/metal/mtl4commandbuffer) instance, such as [makeMachineLearningCommandEncoder()](https://developer.apple.com/documentation/metal/mtl4commandbuffer/makemachinelearningcommandencoder()).

The [dispatchNetwork(intermediatesHeap:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/dispatchnetwork(intermediatesheap:)) method applies to the [machineLearning](https://developer.apple.com/documentation/metal/mtlstages/machinelearning) stage of a machine learning pass. For more information about stages and synchronization, see [MTLStages](https://developer.apple.com/documentation/metal/mtlstages) and [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Configuring the pass
- [setPipelineState(_:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/setpipelinestate(_:)) — Configures the encoder with a machine learning pipeline state instance.
- [setArgumentTable(_:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/setargumenttable(_:)) — Sets an argument table for the command encoder’s machine learning shader stage.

### Running machine learning networks
- [dispatchNetwork(intermediatesHeap:)](https://developer.apple.com/documentation/metal/mtl4machinelearningcommandencoder/dispatchnetwork(intermediatesheap:)) — Dispatches a machine learning network using the current pipeline state and argument table.

## See also

### Encoding a machine learning pass
- [Running a machine learning model on the GPU timeline](https://developer.apple.com/documentation/metal/running-a-machine-learning-model-on-the-gpu-timeline) — Dispatch model inference commands with a machine learning pass in a Metal 4 command buffer.
- [MTL4MachineLearningPipelineState](https://developer.apple.com/documentation/metal/mtl4machinelearningpipelinestate) — A pipeline state that you can use with machine-learning encoder instances.
