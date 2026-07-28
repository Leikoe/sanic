# dispatchThreadgroups(indirectBuffer:threadsPerThreadgroup:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(indirectbuffer:threadsperthreadgroup:)>

Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries, using an indirect buffer for arguments.

## Declaration

```swift
func dispatchThreadgroups(indirectBuffer: MTLGPUAddress, threadsPerThreadgroup: MTLSize)
```

## Parameters

- **indirectBuffer** — GPUAddress of a [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance providing compute parameters. Lay out the data in this buffer as described in the [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) structure. This address requires 4-byte alignment.
- **threadsPerThreadgroup** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in one threadgroup, in each dimension.

## Discussion

This method allows you to supply the threadgroups-per-grid counts indirectly via an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) index. This enables you to calculate this value in the GPU timeline from a shader function, enabling GPU-driven workflows.

Metal assumes that the buffer contents correspond to the layout of struct [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments). You are responsible for ensuring this address aligns to 4-bytes.

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the indirect buffer that the `indirectBuffer` parameter references.

## See also

### Running dispatch commands
- [dispatchThreads(threadsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(threadspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command using an arbitrarily-sized grid.
- [dispatchThreads(indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(indirectbuffer:)) — Encodes a compute dispatch command with an arbitrarily sized grid, using an indirect buffer for arguments.
- [dispatchThreadgroups(threadgroupsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(threadgroupspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries.
