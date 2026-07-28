# MTLArrayType

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlarraytype>

A description of an array.

## Declaration

```swift
class MTLArrayType
```

## Overview

An [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) instance provides details about an array parameter. Don’t create [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) instances directly; other reflection instances contain properties to determine if a parameter is an array and to obtain the [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) instance that describes the array.

## Topics

### Describing the array elements
- [arrayLength](https://developer.apple.com/documentation/metal/mtlarraytype/arraylength) — The number of elements in the array.
- [elementType](https://developer.apple.com/documentation/metal/mtlarraytype/elementtype) — The data type of the array’s elements.
- [stride](https://developer.apple.com/documentation/metal/mtlarraytype/stride) — The stride between array elements, in bytes.
- [argumentIndexStride](https://developer.apple.com/documentation/metal/mtlarraytype/argumentindexstride) — The stride, in bytes, between argument indices.

### Obtaining details for complex array elements
- [element()](https://developer.apple.com/documentation/metal/mtlarraytype/element()) — Provides a description of the underlying type when an array holds other arrays as its elements.
- [elementStructType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementstructtype()) — Provides a description of the underlying struct type when an array holds structs as its elements.
- [elementPointerType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementpointertype()) — Provides a description of the underlying pointer type when an array holds pointers as its elements.
- [elementTextureReferenceType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementtexturereferencetype()) — Provides a description of the underlying texture type when an array holds textures as its elements.

### Instance Methods
- [elementTensorReferenceType()](https://developer.apple.com/documentation/metal/mtlarraytype/elementtensorreferencetype()) — Provides a description of the underlying tensor type when this array holds tensors as its elements.

## See also

### Shader types
- [MTLType](https://developer.apple.com/documentation/metal/mtltype) — A description of a data type.
- [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype) — The parameter type options for GPU functions, such as shaders and compute kernels.
- [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) — A description of a structure.
- [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) — An instance that provides information about a field in a structure.
- [MTLPointerType](https://developer.apple.com/documentation/metal/mtlpointertype) — A description of a pointer.
- [MTLTextureReferenceType](https://developer.apple.com/documentation/metal/mtltexturereferencetype) — A description of a texture.
