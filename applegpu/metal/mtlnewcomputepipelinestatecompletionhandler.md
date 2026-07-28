# MTLNewComputePipelineStateCompletionHandler

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlnewcomputepipelinestatecompletionhandler>

A completion handler signature a method calls when it finishes creating a compute pipeline.

## Declaration

```swift
typealias MTLNewComputePipelineStateCompletionHandler = ((any MTLComputePipelineState)?, (any Error)?) -> Void
```

## Parameters

- **computePipelineState** — An [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) instance if the method completes successfully; otherwise `nil`.
- **error** — On return, if an error occurs, a pointer to an error information instance; otherwise `nil`.

## See also

### Supporting types
- [MTLNewRenderPipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewrenderpipelinestatecompletionhandler) — A completion handler signature a method calls when it finishes creating a render pipeline.
- [MTLNewRenderPipelineStateWithReflectionCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewrenderpipelinestatewithreflectioncompletionhandler) — A completion handler signature a method calls when it finishes creating a render pipeline and reflection information.
- [MTLNewComputePipelineStateWithReflectionCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewcomputepipelinestatewithreflectioncompletionhandler) — A completion handler signature a method calls when it finishes creating a compute pipeline and reflection information.
