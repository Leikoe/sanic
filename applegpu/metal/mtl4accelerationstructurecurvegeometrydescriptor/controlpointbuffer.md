# controlPointBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/controlpointbuffer>

References a buffer containing curve control points.

## Declaration

```swift
var controlPointBuffer: MTL4BufferRange { get set }
```

## Discussion

Control points are interpolated according to the basis function you specify in [curveBasis](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/curvebasis).

You are responsible for ensuring each control is in a format matching the control point format [controlPointFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/controlpointformat) specifies, as well as ensuring that the buffer address of the range is not zero.
