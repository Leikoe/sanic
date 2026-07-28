# drawMeshThreadgroups(threadgroupsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(threadgroupspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)>

Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threadgroups.

## Declaration

```swift
func drawMeshThreadgroups(threadgroupsPerGrid: MTLSize, threadsPerObjectThreadgroup: MTLSize, threadsPerMeshThreadgroup: MTLSize)
```

## Parameters

- **threadgroupsPerGrid** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threadgroups for each grid dimension.
- **threadsPerObjectThreadgroup** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in an object shader threadgroup, if applicable.
- **threadsPerMeshThreadgroup** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in a mesh shader threadgroup.

## See also

### Drawing with meshes
- [drawMeshThreads(threadsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreads(threadspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threads.
- [drawMeshThreadgroups(indirectBuffer:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(indirectbuffer:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with indirect arguments.
