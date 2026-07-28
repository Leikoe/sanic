# threadGroupSizeIsMultipleOfThreadExecutionWidth

*Instance Property · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/threadgroupsizeismultipleofthreadexecutionwidth>

A Boolean value that indicates whether the threadgroup size is always a multiple of the thread execution width.

## Declaration

```swift
var threadGroupSizeIsMultipleOfThreadExecutionWidth: Bool { get set }
```

## Discussion

> **Warning:**
>  When this configuration value is `true` and the threadgroup size isn’t a multiple of thread execution width, the compute pass’s execution results are undefined.

If you can guarantee that the threadgroup size used by all compute commands in this pipeline is a multiple of [threadExecutionWidth](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/threadexecutionwidth), set this property to `true` to take advantage of additional Metal optimizations.

The default value is `false`.

## See also

### Configuring the compute execution environment
- [computeFunction](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/computefunction) — The compute kernel the pipeline calls.
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxtotalthreadsperthreadgroup) — A property that limits the number of threads you can dispatch in a threadgroup for the compute function.
- [maxCallStackDepth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxcallstackdepth) — The maximum call stack depth for indirect function calls in compute shaders.
