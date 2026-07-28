# makeComputePipelineState(descriptor:dynamicLinkingDescriptor:compilerTaskOptions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4compiler/makecomputepipelinestate(descriptor:dynamiclinkingdescriptor:compilertaskoptions:)-7dqdm>

Creates a new compute pipeline state object synchronously.

## Declaration

```swift
func makeComputePipelineState(descriptor: MTL4ComputePipelineDescriptor, dynamicLinkingDescriptor: MTL4PipelineStageDynamicLinkingDescriptor? = nil, compilerTaskOptions: MTL4CompilerTaskOptions? = nil) throws -> any MTLComputePipelineState
```

## Parameters

- **descriptor** — A compute pipeline state descriptor describing the pipeline this compiler creates.
- **compilerTaskOptions** — A description of the compilation process itself, providing parameters that influence execution of the compilation process.

## Return Value

A new compute pipeline state object upon success, otherwise this method throws.
