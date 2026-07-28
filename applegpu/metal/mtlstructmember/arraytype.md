# arrayType()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstructmember/arraytype()>

Provides a description of the underlying array when the struct member holds an array.

## Declaration

```swift
func arrayType() -> MTLArrayType?
```

## Return Value

An object that describes the array. If [dataType](https://developer.apple.com/documentation/metal/mtlstructmember/datatype) indicates that this member is not an array, this method returns `nil.`

## See also

### Obtaining struct member details
- [structType()](https://developer.apple.com/documentation/metal/mtlstructmember/structtype()) — Provides a description of the underlying struct when the struct member holds a struct.
- [pointerType()](https://developer.apple.com/documentation/metal/mtlstructmember/pointertype()) — Provides a description of the underlying pointer when the struct member holds a pointer.
- [textureReferenceType()](https://developer.apple.com/documentation/metal/mtlstructmember/texturereferencetype()) — Provides a description of the underlying texture when the struct member holds a texture.
