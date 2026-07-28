# indexType

*Instance Property · iOS 17.0, iPadOS 17.0, Mac Catalyst 17.0, macOS 14.0, tvOS 17.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indextype>

The size of each index in the index buffer.

## Declaration

```swift
var indexType: MTLIndexType { get set }
```

## Discussion

Set this property to a value that reflects the size of the indices in the [indexBuffer](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor/indexbuffer) property, such as [MTLIndexType.uint16](https://developer.apple.com/documentation/metal/mtlindextype/uint16) or [MTLIndexType.uint32](https://developer.apple.com/documentation/metal/mtlindextype/uint32).
