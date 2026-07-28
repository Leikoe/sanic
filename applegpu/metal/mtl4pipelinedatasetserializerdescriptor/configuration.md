# configuration

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor/configuration>

Specifies the configuration of the serialization process.

## Declaration

```swift
var configuration: MTL4PipelineDataSetSerializerConfiguration { get set }
```

## Discussion

The configuration of the serialization process determines the mechanisms you use to serialize pipeline data sets.

When this configuration contains `MTL4PipelineDataSetSerializerConfigurationCaptureDescriptors`, use `serializeAsPipelinesScriptWithError:` to serialize pipeline scripts.

If this option contains `MTL4PipelineDataSetSerializerConfigurationCaptureBinaries`, the serializer can additionally serialize to a binary archive by calling `serializeAsArchiveAndFlushToURL:error::`.
