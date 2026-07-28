# makeRenderPipelineDescriptorForSpecialization()

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/makerenderpipelinedescriptorforspecialization()>

Creates a render pipeline descriptor from this pipeline that you can use for pipeline specialization.

## Declaration

```swift
func makeRenderPipelineDescriptorForSpecialization() -> MTL4PipelineDescriptor
```

## Return Value

A new pipeline descriptor that you use for pipeline state specialization.

## Discussion

Use this method to obtain a new [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) instance that you can use to specialize any unspecialized properties in this pipeline state object.

The returned descriptor contains every unspecialized field in the current pipeline state object, set to unspecialized. It may, however, not contain valid or accurate properties in any other field.

This descriptor is only valid for the purpose of calling specialization functions on the [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) to specialize this pipeline, for example: [newRenderPipelineStateBySpecializationWithDescriptor:pipeline:error:](https://developer.apple.com/documentation/metal/mtl4compiler/newrenderpipelinestatebyspecializationwithdescriptor:pipeline:error:).

Although this method returns the [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) base class, the concrete instance this method returns corresponds to the specific descriptor type for the creation of this pipeline state, for example if a [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) instance creates this current pipeline form a [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor), this method returns a concrete [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) instance.
