# instanceDescriptorBufferOffset

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbufferoffset>

The offset, in bytes, to the descripton of the first instance.

## Declaration

```swift
var instanceDescriptorBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of 64 bytes. Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Specifying the list of instances
- [instanceCount](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancecount) — The number of instances in the instance descriptor buffer.
- [instanceDescriptorBuffer](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorbuffer) — A buffer that contains descriptions of each instance in the acceleration structure.
- [instanceDescriptorStride](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/instancedescriptorstride) — The stride, in bytes, between instance descriptions.
