# controlPointStride

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointstride>

The stride, in bytes, between control points in the buffer.

## Declaration

```swift
var controlPointStride: Int { get set }
```

## Discussion

The stride needs to be a multiple of the format element size you configure with the [controlPointFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointformat) property, and at least the format’s size. The default value is `0`, which indicates that the control point elements in the buffer have zero bytes of padding between them.
