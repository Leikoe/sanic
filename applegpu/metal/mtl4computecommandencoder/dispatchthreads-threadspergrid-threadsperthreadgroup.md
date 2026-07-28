# dispatchThreads(threadsPerGrid:threadsPerThreadgroup:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(threadspergrid:threadsperthreadgroup:)>

Encodes a compute dispatch command using an arbitrarily-sized grid.

## Declaration

```swift
func dispatchThreads(threadsPerGrid: MTLSize, threadsPerThreadgroup: MTLSize)
```

## Parameters

- **threadsPerGrid** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in the grid, in each dimension.
- **threadsPerThreadgroup** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in one threadgroup, in each dimension.

## See also

### Running dispatch commands
- [dispatchThreads(indirectBuffer:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreads(indirectbuffer:)) — Encodes a compute dispatch command with an arbitrarily sized grid, using an indirect buffer for arguments.
- [dispatchThreadgroups(threadgroupsPerGrid:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(threadgroupspergrid:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries.
- [dispatchThreadgroups(indirectBuffer:threadsPerThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4computecommandencoder/dispatchthreadgroups(indirectbuffer:threadsperthreadgroup:)) — Encodes a compute dispatch command with a grid that aligns to threadgroup boundaries, using an indirect buffer for arguments.
