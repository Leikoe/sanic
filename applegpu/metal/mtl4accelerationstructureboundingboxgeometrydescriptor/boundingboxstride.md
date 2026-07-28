# boundingBoxStride

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor/boundingboxstride>

Assigns the stride, in bytes, between bounding boxes in the bounding box buffer `boundingBoxBuffer` references.

## Declaration

```swift
var boundingBoxStride: Int { get set }
```

## Discussion

You are responsible for ensuring this stride is at least 24 bytes and a multiple of 4 bytes.

This property defaults to `24` bytes.
