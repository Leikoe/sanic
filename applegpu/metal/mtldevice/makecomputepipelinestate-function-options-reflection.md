# makeComputePipelineState(function:options:reflection:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:reflection:)>

Synchronously creates a compute pipeline state and reflection with a function instance.

## Declaration

```swift
func makeComputePipelineState(function computeFunction: any MTLFunction, options: MTLPipelineOption, reflection: AutoreleasingUnsafeMutablePointer<MTLAutoreleasedComputePipelineReflection?>?) throws -> any MTLComputePipelineState
```

## Parameters

- **computeFunction** — An [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance.
- **options** — An [MTLPipelineOption](https://developer.apple.com/documentation/metal/mtlpipelineoption) instance that represents the reflection information you want the method to generate.
- **reflection** — In Swift, an optional pointer to an [MTLAutoreleasedComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedcomputepipelinereflection) optional. In Objective-C, a pointer to an [MTLAutoreleasedComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedcomputepipelinereflection) instance.

## Discussion

Use the compute pipeline state to configure a compute pass by calling the [setComputePipelineState(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/setcomputepipelinestate(_:)) method of an [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) instance.

## See also

### Creating compute pipeline states
- [makeComputePipelineState(descriptor:options:reflection:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(descriptor:options:reflection:)) — Synchronously creates a compute pipeline state and reflection information.
- [makeComputePipelineState(descriptor:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(descriptor:options:completionhandler:)) — Asynchronously creates a compute pipeline state and reflection information.
- [makeComputePipelineState(function:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:)) — Synchronously creates a compute pipeline state with a function instance.
- [makeComputePipelineState(function:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:completionhandler:)) — Asynchronously creates a compute pipeline state with a function instance.
- [makeComputePipelineState(function:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:completionhandler:)) — Asynchronously creates a compute pipeline state and reflection with a function instance.
