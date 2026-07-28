# MTLPipelineOption

*Structure · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpipelineoption>

Options that determine how Metal prepares the pipeline.

## Declaration

```swift
struct MTLPipelineOption
```

## Topics

### Retrieving argument information
- [bufferTypeInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/buffertypeinfo) — An option instance that provides detailed buffer type information for buffer arguments.
- [failOnBinaryArchiveMiss](https://developer.apple.com/documentation/metal/mtlpipelineoption/failonbinaryarchivemiss) — An option that instructs the compiler to return an error when a GPU function isn’t in a binary archive.
- [argumentInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/argumentinfo) — An option instance that provides argument information for textures and threadgroup memory.

### Creating compilation options
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlpipelineoption/init(rawvalue:)) — Creates empty compilation options.

### Type properties
- [bindingInfo](https://developer.apple.com/documentation/metal/mtlpipelineoption/bindinginfo) — An option that provides binding information for pipeline state resources.

## See also

### Configuring a compute pipeline state
- [MTL4ComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor) — Describes a compute pipeline state.
- [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor) — An instance describing the desired GPU state for a kernel call in a compute pass.
- [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) — An interface that represents a GPU pipeline configuration for running kernels in a compute pass.
- [MTLStageInputOutputDescriptor](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor) — A description of the input and output data of a function.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
