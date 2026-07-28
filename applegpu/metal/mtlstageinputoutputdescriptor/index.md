# MTLStageInputOutputDescriptor

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor>

A description of the input and output data of a function.

## Declaration

```swift
class MTLStageInputOutputDescriptor
```

## Topics

### Describing argument layouts
- [attributes](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/attributes) — An array that describes where and how to fetch data for the function.
- [layouts](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/layouts) — An array that describes how the function fetches data.

### Declaring index buffers for indirect compute commands
- [indexBufferIndex](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/indexbufferindex) — The location of the index buffer for a compute function using indexed thread addressing.
- [indexType](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/indextype) — The data type of the indices stored in the index buffer.

### Resetting the descriptor
- [reset()](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor/reset()) — Resets the default state for the descriptor.

## See also

### Configuring a compute pipeline state
- [MTL4ComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor) — Describes a compute pipeline state.
- [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor) — An instance describing the desired GPU state for a kernel call in a compute pass.
- [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) — An interface that represents a GPU pipeline configuration for running kernels in a compute pass.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
