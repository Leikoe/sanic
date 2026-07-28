# elementTextureReferenceType()

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlarraytype/elementtexturereferencetype()>

Provides a description of the underlying texture type when an array holds textures as its elements.

## Declaration

```swift
func elementTextureReferenceType() -> MTLTextureReferenceType?
```

## Return Value

An object that describes the texture. If the array elements aren’t textures, this method returns `nil`.

## Discussion

Use this method if [elementType](https://developer.apple.com/documentation/metal/mtlarraytype/elementtype) is [MTLDataType.texture](https://developer.apple.com/documentation/metal/mtldatatype/texture).

## See also

### Obtaining details for complex array elements
- [element()](https://developer.apple.com/documentation/metal/mtlarraytype/element()) — Provides a description of the underlying type when an array holds other arrays as its elements.
- [elementStructType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementstructtype()) — Provides a description of the underlying struct type when an array holds structs as its elements.
- [elementPointerType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementpointertype()) — Provides a description of the underlying pointer type when an array holds pointers as its elements.
