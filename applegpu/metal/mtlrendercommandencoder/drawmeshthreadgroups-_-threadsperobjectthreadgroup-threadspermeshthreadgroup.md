# drawMeshThreadgroups(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)>

Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threadgroups.

## Declaration

```swift
func drawMeshThreadgroups(_ threadgroupsPerGrid: MTLSize, threadsPerObjectThreadgroup: MTLSize, threadsPerMeshThreadgroup: MTLSize)
```

## Parameters

- **threadgroupsPerGrid** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threadgroups for each grid dimension.
- **threadsPerObjectThreadgroup** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in an object shader threadgroup, if applicable.
- **threadsPerMeshThreadgroup** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in a mesh shader threadgroup.

## See also

### Drawing with meshes
- [drawMeshThreads(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreads(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threads.
- [drawMeshThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(indirectbuffer:indirectbufferoffset:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with indirect arguments.
