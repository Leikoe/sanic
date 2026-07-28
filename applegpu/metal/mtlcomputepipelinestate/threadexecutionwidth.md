# threadExecutionWidth

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/threadexecutionwidth>

The number of threads that the GPU executes simultaneously.

## Declaration

```swift
var threadExecutionWidth: Int { get }
```

## Discussion

For better performance, when dispatching a compute command, make the number of threads in the threadgroup a multiple of `threadExecutionWidth`.

See [Creating threads and threadgroups](https://developer.apple.com/documentation/metal/creating-threads-and-threadgroups) and [Calculating threadgroup and grid sizes](https://developer.apple.com/documentation/metal/calculating-threadgroup-and-grid-sizes) for more information on aligning data, thread width, and threadgroup size.

## See also

### Checking threadgroup attributes
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/maxtotalthreadsperthreadgroup) — The maximum number of threads in a threadgroup that you can dispatch to the pipeline.
- [staticThreadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate/staticthreadgroupmemorylength) — The length, in bytes, of statically allocated threadgroup memory.
