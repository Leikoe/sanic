# instanceDescriptorStride

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorstride>

The stride, in bytes, between instance descriptions.

## Declaration

```swift
var instanceDescriptorStride: Int { get set }
```

## Discussion

The stride needs to be at least 64 bytes and needs to be a multiple of 4 bytes. Defaults to 64 bytes.

## See also

### Specifying the list of instances
- [instanceCount](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancecount) — The number of instances in the instance descriptor buffer.
- [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbuffer) — A buffer that contains descriptions of each instance in the acceleration structure.
- [instanceDescriptorBufferOffset](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbufferoffset) — The offset, in bytes, to the descripton of the first instance.
