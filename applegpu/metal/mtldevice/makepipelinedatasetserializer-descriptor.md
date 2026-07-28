# makePipelineDataSetSerializer(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makepipelinedatasetserializer(descriptor:)>

Creates a new pipeline data set serializer instance from a descriptor.

## Declaration

```swift
func makePipelineDataSetSerializer(descriptor: MTL4PipelineDataSetSerializerDescriptor) -> any MTL4PipelineDataSetSerializer
```

## Parameters

- **descriptor** — A [MTL4PipelineDataSetSerializerDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor) instance that configures the new [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer) instance.

## Return Value

A [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer) instance, or `nil` if the function failed.
