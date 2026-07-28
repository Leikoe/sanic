# makeRenderPipelineState(tileDescriptor:options:)

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.5, visionOS*

<https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:)>

Synchronously creates a tile shader’s render pipeline state and reflection information in a tuple.

## Declaration

```swift
func makeRenderPipelineState(tileDescriptor: MTLTileRenderPipelineDescriptor, options: MTLPipelineOption) throws -> (any MTLRenderPipelineState, MTLRenderPipelineReflection?)
```

## Parameters

- **tileDescriptor** — An [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) instance.
- **options** — An [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) instance that represents the reflection information you want the method to generate.

## Return Value

A tuple with a new [MTLTileRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor) instance and an [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) optional instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## See also

### Creating tile render pipeline states
- [makeRenderPipelineState(tileDescriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:reflection:)) — Synchronously creates a tile shader’s render pipeline state and reflection information.
- [makeRenderPipelineState(tileDescriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makerenderpipelinestate(tiledescriptor:options:completionhandler:)) — Asynchronously creates a tile shader’s render pipeline state and reflection information.
