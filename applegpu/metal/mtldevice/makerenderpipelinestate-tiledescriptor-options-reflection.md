# makeRenderPipelineState(tileDescriptor:options:reflection:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:reflection:)>

Synchronously creates a tile shader’s render pipeline state and reflection information.

## Declaration

```swift
func makeRenderPipelineState(tileDescriptor descriptor: MTLTileRenderPipelineDescriptor, options: MTLPipelineOption, reflection: AutoreleasingUnsafeMutablePointer<MTLAutoreleasedRenderPipelineReflection?>?) throws -> any MTLRenderPipelineState
```

## Parameters

- **descriptor** — An [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) instance.
- **options** — An [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) instance that represents the reflection information you want the method to generate.
- **reflection** — In Swift, an optional pointer to an [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) optional. In Objective-C, a pointer to an [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) instance. Pass `nil` in either language when you don’t need reflection data. Otherwise on return, if the method completes successfully, it assigns an [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) instance to the pointee, which contains the details about the function arguments.

## Return Value

A new [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## See also

### Creating tile render pipeline states
- [makeRenderPipelineState(tileDescriptor:options:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:)) — Synchronously creates a tile shader’s render pipeline state and reflection information in a tuple.
- [makeRenderPipelineState(tileDescriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:completionhandler:)) — Asynchronously creates a tile shader’s render pipeline state and reflection information.
