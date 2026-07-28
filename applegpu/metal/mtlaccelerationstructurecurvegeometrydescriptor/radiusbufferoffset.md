# radiusBufferOffset

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/radiusbufferoffset>

The offset, in bytes, to the radius data in the buffer.

## Declaration

```swift
var radiusBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of the radius format you configure with the [radiusFormat](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/radiusformat) property. You also need to align the offset to both the radius format’s size and the platform’s buffer alignment requirement.
