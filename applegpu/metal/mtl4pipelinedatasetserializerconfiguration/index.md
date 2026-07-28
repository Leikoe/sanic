# MTL4PipelineDataSetSerializerConfiguration

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration>

Configuration options for pipeline dataset serializer objects.

## Declaration

```swift
struct MTL4PipelineDataSetSerializerConfiguration
```

## Overview

Use these options to enable different functionality in instances of [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer).

You can combine these values via a logical `OR` and set it to [configuration](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor/configuration) to specify desired level of serialization support for instances of [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer).

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration/init(rawvalue:))

### Type Properties
- [captureBinaries](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration/capturebinaries) — Enables serializing pipeline binary functions.
- [captureDescriptors](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration/capturedescriptors) — Enables serializing pipeline scripts.

## See also

### Pipeline harvesting
- [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer) — A fast-addition container for collecting data during pipeline state creation.
- [MTL4PipelineDataSetSerializerDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor) — Groups together properties to create a pipeline data set serializer.
- [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) — Base type for descriptors you use for building pipeline state objects.
- [MTL4PipelineOptions](https://developer.apple.com/documentation/metal/mtl4pipelineoptions) — Provides options controlling how to compile a pipeline state.
