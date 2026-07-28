# controlPointBufferOffset

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointbufferoffset>

The offset, in bytes, to the control point data in the buffer.

## Declaration

```swift
var controlPointBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of the format element size you configure with the [controlPointFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/controlpointformat) property. You also need to align the offset to the platform’s buffer alignment requirement.
