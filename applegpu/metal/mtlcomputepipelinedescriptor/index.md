# MTLComputePipelineDescriptor

*Class · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor>

An instance describing the desired GPU state for a kernel call in a compute pass.

## Declaration

```swift
class MTLComputePipelineDescriptor
```

## Overview

> **Important:**
>  Before creating a pipeline state, set the [computeFunction](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/computefunction) property on your descriptor instance. This property tells the GPU which kernel to run.

A pipeline descriptor provides information necessary for creating an [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) instance.

## Topics

### Configuring the compute execution environment
- [computeFunction](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/computefunction) — The compute kernel the pipeline calls.
- [threadGroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/threadgroupsizeismultipleofthreadexecutionwidth) — A Boolean value that indicates whether the threadgroup size is always a multiple of the thread execution width.
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxtotalthreadsperthreadgroup) — A property that limits the number of threads you can dispatch in a threadgroup for the compute function.
- [maxCallStackDepth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxcallstackdepth) — The maximum call stack depth for indirect function calls in compute shaders.

### Configuring compute pass inputs
- [stageInputDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/stageinputdescriptor) — The organization of input and output data for the next kernel call.
- [MTLAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlattributedescriptor) — A descriptor of an argument’s format and where its data is in memory.
- [MTLAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlattributedescriptorarray) — An array of attribute descriptor objects.
- [MTLBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor) — A description of how a compute function fetches input data for an attribute.
- [MTLBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray) — An array of buffer layout descriptor objects.

### Configuring buffer mutability
- [buffers](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/buffers) — The buffer mutability options to apply to the next kernel call.

### Identifying the pipeline state object
- [label](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/label) — A string that identifies the instance.

### Configuring indirect command buffers
- [supportIndirectCommandBuffers](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/supportindirectcommandbuffers) — A Boolean value that indicates whether you can encode commands that reference the pipeline state object into an indirect command buffer.

### Configuring shader validation
- [shaderValidation](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/shadervalidation) — A value that enables or disables shader validation for the pipeline.

### Reset to defaults
- [reset()](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/reset()) — Resets all compute pipeline descriptor properties to their default values.

### Loading dynamic libraries to link at runtime
- [preloadedLibraries](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/preloadedlibraries) — The dynamic libraries that contain precompiled shader functions you want to link.
- [insertLibraries](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/insertlibraries) — The dynamic libraries that contain precompiled shader functions you want to link.

### Setting callable functions
- [linkedFunctions](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/linkedfunctions) — The functions with available function pointers for the next kernel call.

### Loading binary archives
- [supportAddingBinaryFunctions](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/supportaddingbinaryfunctions) — A Boolean value that indicates whether you can use the pipeline to create new pipelines by adding binary functions to its callable functions list.
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/binaryarchives) — The binary archives that contain any precompiled shader functions to link.

### Instance Properties
- [requiredThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/requiredthreadsperthreadgroup)

## See also

### Configuring a compute pipeline state
- [MTL4ComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor) — Describes a compute pipeline state.
- [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) — An interface that represents a GPU pipeline configuration for running kernels in a compute pass.
- [MTLStageInputOutputDescriptor](https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor) — A description of the input and output data of a function.
- [MTLPipelineBufferDescriptor](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor) — The mutability options for a buffer that a render or compute pipeline uses.
- [MTLPipelineBufferDescriptorArray](https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray) — An array of pipeline buffer descriptors.
- [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) — Options that determine how Metal prepares the pipeline.
