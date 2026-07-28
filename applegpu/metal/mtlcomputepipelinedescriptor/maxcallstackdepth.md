# maxCallStackDepth

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxcallstackdepth>

The maximum call stack depth for indirect function calls in compute shaders.

## Declaration

```swift
var maxCallStackDepth: Int { get set }
```

## Discussion

The property’s default value is `1`. Change its value if you use recursive functions in your compute pass.

The maximum call stack depth applies only to indirect function calls in your shader, and affects the upper bound of stack memory for each thread. Indirect function calls include those to visible functions, intersection functions, and to dynamic libraries.

> **Tip:**
>  To avoid a runtime performance impact, keep this value as small as possible because the framework reserves a large call stack.

## See also

### Configuring the compute execution environment
- [computeFunction](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/computefunction) — The compute kernel the pipeline calls.
- [threadGroupSizeIsMultipleOfThreadExecutionWidth](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/threadgroupsizeismultipleofthreadexecutionwidth) — A Boolean value that indicates whether the threadgroup size is always a multiple of the thread execution width.
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/maxtotalthreadsperthreadgroup) — A property that limits the number of threads you can dispatch in a threadgroup for the compute function.
