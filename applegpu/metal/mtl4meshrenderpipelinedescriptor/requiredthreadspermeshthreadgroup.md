# requiredThreadsPerMeshThreadgroup

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4meshrenderpipelinedescriptor/requiredthreadspermeshthreadgroup>

Controls the required number of mesh threads-per-threadgroup when drawing with a mesh shader pipeline you create from this descriptor.

## Declaration

```swift
var requiredThreadsPerMeshThreadgroup: MTLSize { get set }
```

## Discussion

This argument is optional, unless this pipeline uses `CooperativeTensors`, in which case you are responsible for providing it.

When this value is set to non-zero, you are responsible for ensuring the parameter `threadsPerMeshThreadgroup` in any mesh dispatch draw calls that use this mesh render pipeline, such as [drawMeshThreadgroups(threadgroupsPerGrid:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:)](https://developer.apple.com/documentation/metal/mtl4rendercommandencoder/drawmeshthreadgroups(threadgroupspergrid:threadsperobjectthreadgroup:threadspermeshthreadgroup:)), match it.

Setting this value to a size of 0 in every dimension disables this property.
