# dispatchThreads(indirectBuffer:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(indirectbuffer:)>

Encodes a compute dispatch command with an arbitrarily sized grid, using an indirect buffer for arguments.

## Declaration

```swift
func dispatchThreads(indirectBuffer: MTLGPUAddress)
```

## Parameters

- **indirectBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance providing arguments. Lay out the data in this buffer as described in the [MTLDispatchThreadsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadsindirectarguments) structure. This address requires 4-byte alignment.

## See also

### Running dispatch commands
- [dispatchThreads(threadsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(threadspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command using an arbitrarily-sized grid.
- [dispatchThreadgroups(threadgroupsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(threadgroupspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries.
- [dispatchThreadgroups(indirectBuffer:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(indirectbuffer:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries, using an indirect buffer for arguments.
