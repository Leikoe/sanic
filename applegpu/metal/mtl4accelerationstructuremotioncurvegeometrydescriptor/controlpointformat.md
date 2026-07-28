# controlPointFormat

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor/controlpointformat>

Declares the format of the control points in the buffers that the control point buffers reference.

## Declaration

```swift
var controlPointFormat: MTLAttributeFormat { get set }
```

## Discussion

All keyframes share the same control point format. Defaults to `MTLAttributeFormatFloat3`, representing 3 floating point values tightly packed.
