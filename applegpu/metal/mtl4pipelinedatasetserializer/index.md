# MTL4PipelineDataSetSerializer

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer>

A fast-addition container for collecting data during pipeline state creation.

## Declaration

```swift
protocol MTL4PipelineDataSetSerializer : NSObjectProtocol
```

## Overview

Pipeline data serializer instances allow you to create binary archives and serialize pipeline scripts to use with the offline Metal binary generator (`metal-tt`) doc:compiling-binary-archives-from-a-custom-configuration-script.md.

You capture and retain all relevant data for all pipelines a compiler instance creates by providing an instance of this object to its [MTL4CompilerDescriptor](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor).

After capturing data, you can serialize it to a binary archive to persist its contents offline by calling [serializeAsArchiveAndFlush(url:)](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer/serializeasarchiveandflush(url:)). You can also serialize a pipeline script suitable for the offline binary generator (`metal-tt`) by calling [serializeAsPipelinesScript()](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer/serializeaspipelinesscript())

> **Note:**
> The objects [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer) contains are opaque and can’t accelerate compilation for compilers they are not attached to. Additionally, your program can’t read data out of data set serializer instances.

## Topics

### Instance Methods
- [serializeAsArchiveAndFlush(url:)](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer/serializeasarchiveandflush(url:)) — Serializes a pipeline data set to an archive.
- [serializeAsPipelinesScript()](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer/serializeaspipelinesscript()) — Serializes a serializer data set to a pipeline script as raw data.

## See also

### Pipeline harvesting
- [MTL4PipelineDataSetSerializerConfiguration](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration) — Configuration options for pipeline dataset serializer objects.
- [MTL4PipelineDataSetSerializerDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor) — Groups together properties to create a pipeline data set serializer.
- [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) — Base type for descriptors you use for building pipeline state objects.
- [MTL4PipelineOptions](https://developer.apple.com/documentation/metal/mtl4pipelineoptions) — Provides options controlling how to compile a pipeline state.
