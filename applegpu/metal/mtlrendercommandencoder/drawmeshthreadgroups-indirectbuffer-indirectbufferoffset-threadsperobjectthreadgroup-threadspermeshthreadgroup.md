# drawMeshThreadgroups(indirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(indirectbuffer:indirectbufferoffset:threadsperobjectthreadgroup:threadspermeshthreadgroup:)>

Encodes a draw command that invokes a mesh shader and, optionally, an object shader with indirect arguments.

## Declaration

```swift
func drawMeshThreadgroups(indirectBuffer: any MTLBuffer, indirectBufferOffset: Int, threadsPerObjectThreadgroup: MTLSize, threadsPerMeshThreadgroup: MTLSize)
```

## Parameters

- **indirectBuffer** — An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance with data that matches the layout of the [MTLDispatchThreadgroupsIndirectArguments](https://developer.apple.com/documentation/metal/mtldispatchthreadgroupsindirectarguments) structure.
- **indirectBufferOffset** — An integer that represents the location, in bytes, from the start of `indirectBuffer` where the indirect arguments structure begins. See the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) to check for offset alignment requirements for buffers in `device` and `constant` address space.
- **threadsPerObjectThreadgroup** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in an object shader threadgroup, if applicable.
- **threadsPerMeshThreadgroup** — An [MTLSize](https://developer.apple.com/documentation/metal/mtlsize) instance that represents the number of threads in a mesh shader threadgroup.

## See also

### Drawing with meshes
- [drawMeshThreads(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreads(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threads.
- [drawMeshThreadgroups(_:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/drawmeshthreadgroups(_:threadsperobjectthreadgroup:threadspermeshthreadgroup:)) — Encodes a draw command that invokes a mesh shader and, optionally, an object shader with a grid of threadgroups.
