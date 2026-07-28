# MTLComputePipelineState

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate>

An interface that represents a GPU pipeline configuration for running kernels in a compute pass.

## Declaration

```swift
protocol MTLComputePipelineState : MTLAllocation, Sendable
```

## Overview

The [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) protocol is an interface that represents a specific configuration for the GPU pipeline for a compute pass. Use a pipeline state to configure a compute pass by calling the [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)) method of an [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance.

To create a pipeline state, call the appropriate [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) method (see [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation)). You typically make pipeline states at a noncritical time, like when your app first launches. This is because graphics drivers may need time to evaluate and build each pipeline state. However, you can quickly use and reuse each pipeline state throughout your app’s lifetime.

## Topics

### Identifying a pipeline state
- [device](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/device) — The device instance that created the pipeline state.
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/gpuresourceid) — An unique identifier that represents the pipeline state, which you can add to an argument buffer.
- [label](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/label) — A string that helps you identify the compute pipeline state during debugging.

### Checking threadgroup attributes
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/maxtotalthreadsperthreadgroup) — The maximum number of threads in a threadgroup that you can dispatch to the pipeline.
- [threadExecutionWidth](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/threadexecutionwidth) — The number of threads that the GPU executes simultaneously.
- [staticThreadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/staticthreadgroupmemorylength) — The length, in bytes, of statically allocated threadgroup memory.

### Checking imageblock attributes
- [imageblockMemoryLength(forDimensions:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/imageblockmemorylength(fordimensions:)) — Returns the length of reserved memory for an imageblock of a given size.

### Checking indirect command buffer support
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/supportindirectcommandbuffers) — A Boolean value that indicates whether the compute pipeline supports indirect command buffers.

### Checking shader validation
- [shaderValidation](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/shadervalidation) — The current state of shader validation for the pipeline.

### Creating function handles
- [functionHandle(function:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/functionhandle(function:)-7d523) — Creates a function handle for a visible function.

### Adding visible functions
- [makeComputePipelineStateWithAdditionalBinaryFunctions(functions:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makecomputepipelinestatewithadditionalbinaryfunctions(functions:)) — Creates a new pipeline state object with additional callable functions.

### Creating function tables
- [makeVisibleFunctionTable(descriptor:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makevisiblefunctiontable(descriptor:)) — Creates a new visible function table.
- [makeIntersectionFunctionTable(descriptor:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makeintersectionfunctiontable(descriptor:)) — Creates a new intersection function table.

### Instance Properties
- [reflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/reflection) — The compute pipeline’s reflection information, if available.
- [requiredThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/requiredthreadsperthreadgroup)

### Instance Methods
- [functionHandle(function:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/functionhandle(function:)-8spaa) — Gets the function handle for a function this pipeline links at the binary level.
- [functionHandle(withName:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/functionhandle(withname:)) — Gets the function handle for a function this pipeline links at the Metal IR level by name.
- [makeComputePipelineState(additionalBinaryFunctions:)](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/makecomputepipelinestate(additionalbinaryfunctions:)) — Allocates a new compute pipeline state by adding binary functions to this pipeline state.

## See also

### Configuring a compute pipeline state
- [MTL4ComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor) — Describes a compute pipeline state.
- [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor) — An instance describing the desired GPU state for a kernel call in a compute pass.
- [MTLStageInputOutputDescriptor](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor) — A description of the input and output data of a function.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
