# MTL4ComputePipelineDescriptor

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor>

Describes a compute pipeline state.

## Declaration

```swift
class MTL4ComputePipelineDescriptor
```

## Topics

### Instance Properties
- [computeFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/computefunctiondescriptor) — A descriptor representing the compute pipeline’s function.
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/maxtotalthreadsperthreadgroup) — The maximum total number of threads that Metal can execute in a single threadgroup for the compute function.
- [requiredThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/requiredthreadsperthreadgroup) — The required number of threads per threadgroup for compute dispatches.
- [staticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/staticlinkingdescriptor) — An object that contains information about functions to link to the compute pipeline.
- [supportBinaryLinking](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/supportbinarylinking) — A boolean value indicating whether the compute pipeline supports linking binary functions.
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/supportindirectcommandbuffers) — A value indicating whether the pipeline supports Metal indirect command buffers.
- [threadGroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/threadgroupsizeismultipleofthreadexecutionwidth) — A boolean value indicating whether each dimension of the threadgroup size is a multiple of its corresponding thread execution width.

### Instance Methods
- [reset()](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/reset()) — Resets the descriptor to its default values.

## See also

### Configuring a compute pipeline state
- [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor) — An instance describing the desired GPU state for a kernel call in a compute pass.
- [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) — An interface that represents a GPU pipeline configuration for running kernels in a compute pass.
- [MTLStageInputOutputDescriptor](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor) — A description of the input and output data of a function.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
