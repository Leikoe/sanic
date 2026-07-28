# boundingBoxBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor/boundingboxbuffer>

References a buffer containing bounding box data in `MTLAxisAlignedBoundingBoxes` format.

## Declaration

```swift
var boundingBoxBuffer: MTL4BufferRange { get set }
```

## Discussion

You are responsible for ensuring the buffer address of the range is not zero.
