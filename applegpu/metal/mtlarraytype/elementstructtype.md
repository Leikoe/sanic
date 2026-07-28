# elementStructType()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlarraytype/elementstructtype()>

Provides a description of the underlying struct type when an array holds structs as its elements.

## Declaration

```swift
func elementStructType() -> MTLStructType?
```

## Return Value

An object that describes the struct. If the array elements aren’t structs, this method returns `nil`.

## Discussion

Use this method if [elementType](https://developer.apple.com/documentation/metal/mtlarraytype/elementtype) is [MTLDataType.struct](https://developer.apple.com/documentation/metal/mtldatatype/struct).

## See also

### Obtaining details for complex array elements
- [element()](https://developer.apple.com/documentation/metal/mtlarraytype/element()) — Provides a description of the underlying type when an array holds other arrays as its elements.
- [elementPointerType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementpointertype()) — Provides a description of the underlying pointer type when an array holds pointers as its elements.
- [elementTextureReferenceType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementtexturereferencetype()) — Provides a description of the underlying texture type when an array holds textures as its elements.
