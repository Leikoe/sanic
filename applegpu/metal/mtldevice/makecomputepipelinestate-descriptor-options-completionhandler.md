# makeComputePipelineState(descriptor:options:completionHandler:)

*Instance Method · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(descriptor:options:completionhandler:)>

Asynchronously creates a compute pipeline state and reflection information.

## Declaration

```swift
func makeComputePipelineState(descriptor: MTLComputePipelineDescriptor, options: MTLPipelineOption, completionHandler: @escaping @Sendable ((any MTLComputePipelineState)?, MTLComputePipelineReflection?, (any Error)?) -> Void)
```

```swift
func makeComputePipelineState(descriptor: MTLComputePipelineDescriptor, options: MTLPipelineOption) async throws -> (any MTLComputePipelineState, MTLComputePipelineReflection?)
```

## Parameters

- **descriptor** — An [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor) instance.
- **options** — An [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) instance that represents the reflection information you want the method to generate.
- **completionHandler** — A Swift closure or an Objective-C block the method calls when it finishes creating the compute pipeline state.

## Discussion

Use the compute pipeline state to configure a compute pass by calling the [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)) method of an [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance.

## See also

### Creating compute pipeline states
- [makeComputePipelineState(descriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(descriptor:options:reflection:)) — Synchronously creates a compute pipeline state and reflection information.
- [makeComputePipelineState(function:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:)) — Synchronously creates a compute pipeline state with a function instance.
- [makeComputePipelineState(function:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:completionhandler:)) — Asynchronously creates a compute pipeline state with a function instance.
- [makeComputePipelineState(function:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:reflection:)) — Synchronously creates a compute pipeline state and reflection with a function instance.
- [makeComputePipelineState(function:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:completionhandler:)) — Asynchronously creates a compute pipeline state and reflection with a function instance.
