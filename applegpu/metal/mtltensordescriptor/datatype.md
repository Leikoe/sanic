# dataType

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtltensordescriptor/datatype>

The data format of all elements in the data plane.

## Declaration

```swift
var dataType: MTLTensorDataType { get set }
```

## Discussion

The default value of this property is [MTLTensorDataType.float32](https://developer.apple.com/documentation/metal/mtltensordatatype/float32).

[MTLTensorDataType.metalFloat8ue8m0](https://developer.apple.com/documentation/metal/mtltensordatatype/metalfloat8ue8m0) is not a valid data type for this property.
