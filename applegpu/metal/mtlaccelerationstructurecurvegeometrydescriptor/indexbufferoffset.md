# indexBufferOffset

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indexbufferoffset>

The offset, in bytes, to the index data in the buffer.

## Declaration

```swift
var indexBufferOffset: Int { get set }
```

## Discussion

The offset needs to be a multiple of the index data type you configure with the [indexType](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indextype) property. You also need to align the offset to both the index type’s size and the platform’s buffer alignment requirement.
