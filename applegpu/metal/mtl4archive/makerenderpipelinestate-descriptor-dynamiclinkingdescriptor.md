# makeRenderPipelineState(descriptor:dynamicLinkingDescriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4archive/makerenderpipelinestate(descriptor:dynamiclinkingdescriptor:)>

Creates a render pipeline state from the archive with a render descriptor and a dynamic linking descriptor.

## Declaration

```swift
func makeRenderPipelineState(descriptor: MTL4PipelineDescriptor, dynamicLinkingDescriptor: MTL4RenderPipelineDynamicLinkingDescriptor? = nil) throws -> any MTLRenderPipelineState
```

## Parameters

- **descriptor** — A render pipeline descriptor.
- **dynamicLinkingDescriptor** — A descriptor that provides additional properties to link other functions with the pipeline.

## Return Value

A compute pipeline state object upon success, otherwise this function throws.

## Discussion

You create any kind of render pipeline states with this method, including:

- Traditional render pipelines

- Mesh render pipelines

- Tile render pipelines
