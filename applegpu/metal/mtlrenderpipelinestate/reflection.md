# reflection

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtlrenderpipelinestate/reflection>

The render pipeline’s reflection information, if available.

## Declaration

```swift
var reflection: MTLRenderPipelineReflection? { get }
```

## Discussion

The property is `nil` by default to help reduce your app’s memory footprint, but you can create reflection information when your app needs it.

Create reflection information by building a pipeline from an [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) instance with the following steps:

1. Configure the [shaderReflection](https://developer.apple.com/documentation/metal/mtl4pipelineoptions/shaderreflection) property of an [MTL4PipelineOptions](https://developer.apple.com/documentation/metal/mtl4pipelineoptions) instance.

2. Assign that instance to the [options](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor/options) property of an [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) instance.

3. Create a compute pipeline state by passing that pipeline descriptor to one of the [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) instance’s methods.

The property is `nil` when you create a pipeline state from an[MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance, such as with its [makeRenderPipelineState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:)) method.
