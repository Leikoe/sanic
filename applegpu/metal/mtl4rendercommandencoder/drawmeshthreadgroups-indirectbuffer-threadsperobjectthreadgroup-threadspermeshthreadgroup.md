# drawMeshThreadgroups(indirectBuffer:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(indirectbuffer:threadsperobjectthreadgroup:threadspermeshthreadgroup:)>

Encodes a draw command that invokes a mesh shader and, optionally, an object shader with indirect arguments.

## Declaration

```swift
func drawMeshThreadgroups(indirectBuffer: MTLGPUAddress, threadsPerObjectThreadgroup: MTLSize, threadsPerMeshThreadgroup: MTLSize)
```

## Parameters

- **indirectBuffer** — GPUAddress of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) structure. This address requires 4-byte alignment.
- **threadsPerObjectThreadgroup** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in an object shader threadgroup, if applicable.
- **threadsPerMeshThreadgroup** — A [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in a mesh shader threadgroup.

## Discussion

This method enables you to determine the number of threadgroups per grid indirectly, in the GPU timeline. Metal expects this buffer’s contents to match the layout of structure [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments). You are responsible for ensuring the address of this buffer has 4-byte alignment.

Use an instance of [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) to mark residency of the indirect buffer that the `indirectBuffer` parameter references.

## See also

### Drawing with meshes
- [drawMeshThreads(threadsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreads(threadspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threads.
- [drawMeshThreadgroups(threadgroupsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(threadgroupspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threadgroups.
