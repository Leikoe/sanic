# MTLPipelineBufferDescriptor

*Class · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor>

The mutability options for a buffer that a render or compute pipeline uses.

## Declaration

```swift
class MTLPipelineBufferDescriptor
```

## Overview

Metal can perform additional optimizations if you guarantee that neither the CPU nor the GPU modify a buffer’s contents before starting a pass. Use immutable buffers as much as possible to take advantage of Metal optimizations.

To declare that a buffer is immutable, set the [mutability](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/mutability) property of their associated [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) object to [MTLMutability.immutable](https://developer.apple.com/documentation/metal/mtlmutability/immutable).

## Topics

### Setting buffer mutability
- [mutability](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor/mutability) — A mutability option that determines whether you can update a buffer’s contents before related commands use the buffer.
- [MTLMutability](https://developer.apple.com/documentation/metal/mtlmutability) — The options that determine the mutability of a buffer’s contents.

## See also

### Configuring a compute pipeline state
- [MTL4ComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor) — Describes a compute pipeline state.
- [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor) — An instance describing the desired GPU state for a kernel call in a compute pass.
- [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) — An interface that represents a GPU pipeline configuration for running kernels in a compute pass.
- [MTLStageInputOutputDescriptor](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor) — A description of the input and output data of a function.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
