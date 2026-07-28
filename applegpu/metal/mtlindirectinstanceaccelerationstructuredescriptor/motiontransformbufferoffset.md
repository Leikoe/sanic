# motionTransformBufferOffset

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor/motiontransformbufferoffset>

The offset, in bytes, to the descripton of the first motion transform.

## Declaration

```swift
var motionTransformBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of 64 bytes. Check the [Metal feature set tables (PDF)](https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf) for potential alignment restrictions.
