# radiusStride

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusstride>

Sets the stride, in bytes, between radii in the radius buffer.

## Declaration

```swift
var radiusStride: Int { get set }
```

## Discussion

You are responsible for ensuring this property is set to a multiple of the size corresponding to the [radiusFormat](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/radiusformat). All keyframes share the same radius stride.

This property defaults to `0` bytes, indicating that the radii are tightly packed.
