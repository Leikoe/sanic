# computeFunction

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/computefunction>

The compute kernel the pipeline calls.

## Declaration

```swift
var computeFunction: (any MTLFunction)? { get set }
```

## Discussion

> **Warning:**
>  Ensure that this value is non-`nil` before creating a new [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) with the associated pipeline descriptor instance.

The default value is `nil`.

## See also

### Configuring the compute execution environment
- [threadGroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/threadgroupsizeismultipleofthreadexecutionwidth) — A Boolean value that indicates whether the threadgroup size is always a multiple of the thread execution width.
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxtotalthreadsperthreadgroup) — A property that limits the number of threads you can dispatch in a threadgroup for the compute function.
- [maxCallStackDepth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxcallstackdepth) — The maximum call stack depth for indirect function calls in compute shaders.
