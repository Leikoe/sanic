# makeRenderPipelineState(descriptor:options:completionHandler:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:completionhandler:)-5gdww>

Asynchronously creates a render pipeline state and reflection information.

## Declaration

```swift
func makeRenderPipelineState(descriptor: MTLRenderPipelineDescriptor, options: MTLPipelineOption, completionHandler: @escaping @Sendable ((any MTLRenderPipelineState)?, MTLRenderPipelineReflection?, (any Error)?) -> Void)
```

```swift
func makeRenderPipelineState(descriptor: MTLRenderPipelineDescriptor, options: MTLPipelineOption) async throws -> (any MTLRenderPipelineState, MTLRenderPipelineReflection?)
```

## Parameters

- **descriptor** — An [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) instance.
- **options** — An [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) instance that represents the reflection information you want the method to generate.
- **completionHandler** — A Swift closure or an Objective-C block the method calls when it finishes creating the render pipeline state.

## Discussion

Use the graphics-rendering pipeline state to configure a render pass by calling the [setRenderPipelineState(_:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/setrenderpipelinestate(_:)) method of an [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) instance.

## See also

### Creating render pipeline states with vertex shaders
- [makeRenderPipelineState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:)) — Synchronously creates a render pipeline state.
- [makeRenderPipelineState(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:completionhandler:)) — Asynchronously creates a render pipeline state.
- [makeRenderPipelineState(descriptor:options:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:)-89vxc) — Synchronously creates a render pipeline state and reflection information in a tuple.
- [makeRenderPipelineState(descriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(descriptor:options:reflection:)) — Synchronously creates a render pipeline state and reflection information.
