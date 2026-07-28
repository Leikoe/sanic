# makeRenderPipelineState(descriptor:options:reflection:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:reflection:)>

Synchronously creates a render pipeline state and reflection information.

## Declaration

```swift
func makeRenderPipelineState(descriptor: MTLRenderPipelineDescriptor, options: MTLPipelineOption, reflection: AutoreleasingUnsafeMutablePointer<MTLAutoreleasedRenderPipelineReflection?>?) throws -> any MTLRenderPipelineState
```

## Parameters

- **descriptor** — An [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) instance.
- **options** — An [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) instance that represents the reflection information you want the method to generate.
- **reflection** — In Swift, an optional pointer to an [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) optional. In Objective-C, a pointer to an [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) instance. Pass `nil` in either language when you don’t need reflection data. Otherwise on return, if the method completes successfully, it assigns an [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) instance to the pointee, which contains the details about the function arguments.

## Return Value

A new [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

Use the graphics-rendering pipeline state to configure a render pass by calling the [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) method of an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance.

## See also

### Creating render pipeline states with vertex shaders
- [makeRenderPipelineState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:)) — Synchronously creates a render pipeline state.
- [makeRenderPipelineState(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:completionhandler:)) — Asynchronously creates a render pipeline state.
- [makeRenderPipelineState(descriptor:options:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:)-89vxc) — Synchronously creates a render pipeline state and reflection information in a tuple.
- [makeRenderPipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:completionhandler:)-5gdww) — Asynchronously creates a render pipeline state and reflection information.
