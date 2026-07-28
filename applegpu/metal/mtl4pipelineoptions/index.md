# MTL4PipelineOptions

*Class · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4pipelineoptions>

Provides options controlling how to compile a pipeline state.

## Declaration

```swift
class MTL4PipelineOptions
```

## Overview

You provide these options through the [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) class at compilation time.

## Topics

### Instance Properties
- [shaderReflection](https://developer.apple.com/documentation/metal/mtl4pipelineoptions/shaderreflection) — Controls whether to include Metal shader reflection in this pipeline.
- [shaderValidation](https://developer.apple.com/documentation/metal/mtl4pipelineoptions/shadervalidation) — Controls whether to enable or disable Metal Shader Validation for the pipeline.

## See also

### Pipeline harvesting
- [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer) — A fast-addition container for collecting data during pipeline state creation.
- [MTL4PipelineDataSetSerializerConfiguration](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration) — Configuration options for pipeline dataset serializer objects.
- [MTL4PipelineDataSetSerializerDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor) — Groups together properties to create a pipeline data set serializer.
- [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) — Base type for descriptors you use for building pipeline state objects.
