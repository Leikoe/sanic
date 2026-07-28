# instanceDescriptorBuffer

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbuffer>

A buffer that contains descriptions of each instance in the acceleration structure.

## Declaration

```swift
var instanceDescriptorBuffer: (any MTLBuffer)? { get set }
```

## Discussion

You need to set a buffer before creating the instanced acceleration structure. The buffer needs to contain a list of instance data structures, each defining the characteristics of an instance. The descriptor’s [instanceDescriptorType](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptortype) property determines which memory layout to use for the instance data; see [MTLAccelerationStructureInstanceDescriptorType](https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype) for more information.

## See also

### Specifying the list of instances
- [instanceCount](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancecount) — The number of instances in the instance descriptor buffer.
- [instanceDescriptorBufferOffset](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbufferoffset) — The offset, in bytes, to the descripton of the first instance.
- [instanceDescriptorStride](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorstride) — The stride, in bytes, between instance descriptions.
