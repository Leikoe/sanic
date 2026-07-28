# radiusBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/radiusbuffer>

Assigns a reference to a buffer containing the curve radius for each control point.

## Declaration

```swift
var radiusBuffer: MTL4BufferRange { get set }
```

## Discussion

Metal interpolates curve radii according to the basis function you specify via [curveBasis](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/curvebasis).

You are responsible for ensuring the type of each radius matches the type property [radiusFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/radiusformat) specifies, that each radius is at least zero, and that the buffer address of the range is not zero.
