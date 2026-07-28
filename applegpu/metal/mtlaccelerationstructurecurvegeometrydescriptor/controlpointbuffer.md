# controlPointBuffer

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointbuffer>

A buffer that contains curve control points.

## Declaration

```swift
var controlPointBuffer: (any MTLBuffer)? { get set }
```

## Discussion

You provide control points in the format that matches the [controlPointFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointformat) property. This property needs to have a non-nil value when you build an acceleration structure.
