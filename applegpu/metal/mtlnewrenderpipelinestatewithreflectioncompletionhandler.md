# MTLNewRenderPipelineStateWithReflectionCompletionHandler

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlnewrenderpipelinestatewithreflectioncompletionhandler>

A completion handler signature a method calls when it finishes creating a render pipeline and reflection information.

## Declaration

```swift
typealias MTLNewRenderPipelineStateWithReflectionCompletionHandler = ((any MTLRenderPipelineState)?, MTLRenderPipelineReflection?, (any Error)?) -> Void
```

## Parameters

- **renderPipelineState** — An [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) instance if the method successfully compiles the library without any errors; otherwise `nil`.
- **reflection** — An [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) instance if the method completes successfully; otherwise `nil`.
- **error** — If an error occurs, an error information instance; otherwise `nil`.

## See also

### Supporting types
- [MTLNewRenderPipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewrenderpipelinestatecompletionhandler) — A completion handler signature a method calls when it finishes creating a render pipeline.
- [MTLNewComputePipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewcomputepipelinestatecompletionhandler) — A completion handler signature a method calls when it finishes creating a compute pipeline.
- [MTLNewComputePipelineStateWithReflectionCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewcomputepipelinestatewithreflectioncompletionhandler) — A completion handler signature a method calls when it finishes creating a compute pipeline and reflection information.
