# supportAttributeStrides

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor/supportattributestrides>

Controls whether Metal should reserve memory for attribute strides in the argument table.

## Declaration

```swift
var supportAttributeStrides: Bool { get set }
```

## Discussion

Set this value to true if you intend to provide dynamic attribute strides when binding vertex array buffers to the argument table by calling [setAddress(_:attributeStride:index:)](https://developer.apple.com/documentation/metal/mtl4argumenttable/setaddress(_:attributestride:index:))

The default value of this property is [false](https://developer.apple.com/documentation/Swift/false).
