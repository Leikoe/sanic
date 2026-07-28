# motionTransformBufferOffset

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformbufferoffset>

The offset, in bytes, to the descripton of the first motion transform.

## Declaration

```swift
var motionTransformBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of 64 bytes. Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.

## See also

### Specifying motion data
- [motionTransformCount](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformcount) — The number of motion transforms in the motion transform buffer.
- [motionTransformBuffer](https://developer.apple.com/documentation/metal/mtlinstanceaccelerationstructuredescriptor/motiontransformbuffer) — A buffer that contains descriptions of each motion transform in the acceleration structure.
