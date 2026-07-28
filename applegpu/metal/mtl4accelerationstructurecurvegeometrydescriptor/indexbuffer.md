# indexBuffer

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4accelerationstructurecurvegeometrydescriptor/indexbuffer>

Assigns an optional index buffer containing references to control points in the control point buffer.

## Declaration

```swift
var indexBuffer: MTL4BufferRange { get set }
```

## Discussion

Each index represents the first control point of a curve segment. You are responsible for ensuring the buffer address of the range is not zero.
