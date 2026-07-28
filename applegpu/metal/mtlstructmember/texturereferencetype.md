# textureReferenceType()

*Instance Method · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstructmember/texturereferencetype()>

Provides a description of the underlying texture when the struct member holds a texture.

## Declaration

```swift
func textureReferenceType() -> MTLTextureReferenceType?
```

## Return Value

An object that describes the texture. If [dataType](https://developer.apple.com/documentation/metal/mtlstructmember/datatype) indicates that this member isn’t a texture, this method returns `nil`.

## See also

### Obtaining struct member details
- [arrayType()](https://developer.apple.com/documentation/metal/mtlstructmember/arraytype()) — Provides a description of the underlying array when the struct member holds an array.
- [structType()](https://developer.apple.com/documentation/metal/mtlstructmember/structtype()) — Provides a description of the underlying struct when the struct member holds a struct.
- [pointerType()](https://developer.apple.com/documentation/metal/mtlstructmember/pointertype()) — Provides a description of the underlying pointer when the struct member holds a pointer.
