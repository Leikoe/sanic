# indexBuffer

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indexbuffer>

A buffer that contains references to control points in the control point buffer.

## Declaration

```swift
var indexBuffer: (any MTLBuffer)? { get set }
```

## Discussion

This property needs to have a non-nil value when you build an acceleration structure.
