# MTLPointerType

*Class · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlpointertype>

A description of a pointer.

## Declaration

```swift
class MTLPointerType
```

## Topics

### Describing the pointer elements
- [alignment](https://developer.apple.com/documentation/metal/mtlpointertype/alignment) — The required byte alignment in memory for the element data.
- [dataSize](https://developer.apple.com/documentation/metal/mtlpointertype/datasize) — The size, in bytes, of the element data.
- [elementType](https://developer.apple.com/documentation/metal/mtlpointertype/elementtype) — The data type of the element data.
- [access](https://developer.apple.com/documentation/metal/mtlpointertype/access) — The function’s read/write access to the element data.
- [elementIsArgumentBuffer](https://developer.apple.com/documentation/metal/mtlpointertype/elementisargumentbuffer) — A Boolean value that indicates whether the element is an argument buffer.

### Obtaining details for complex pointer elements
- [elementArrayType()](https://developer.apple.com/documentation/metal/mtlpointertype/elementarraytype()) — Provides a description of the underlying array when the pointer points to an array.
- [elementStructType()](https://developer.apple.com/documentation/metal/mtlpointertype/elementstructtype()) — Provides a description of the underlying struct when the pointer points to a struct.

## See also

### Shader types
- [MTLType](https://developer.apple.com/documentation/metal/mtltype) — A description of a data type.
- [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype) — The parameter type options for GPU functions, such as shaders and compute kernels.
- [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) — A description of an array.
- [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) — A description of a structure.
- [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) — An instance that provides information about a field in a structure.
- [MTLTextureReferenceType](https://developer.apple.com/documentation/metal/mtltexturereferencetype) — A description of a texture.
