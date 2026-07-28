# MTLStructMember

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstructmember>

An instance that provides information about a field in a structure.

## Declaration

```swift
class MTLStructMember
```

## Overview

[MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) is part of the reflection API that allows Metal framework code to query details about an argument of a Metal shading language function. An [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) instance describes the data type of one field in a struct that is passed as an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) argument, which is represented by [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument).

Don’t create [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) instances directly. You obtain an [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) instance from either the [members](https://developer.apple.com/documentation/metal/mtlstructtype/members) property or the [memberByName(_:)](https://developer.apple.com/documentation/metal/mtlstructtype/memberbyname(_:)) method of an [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) instance. The [dataType](https://developer.apple.com/documentation/metal/mtlstructmember/datatype) property of the [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) instance tells you what kind of data is stored in the member. Recursively drill down every struct member until you reach a data type that is neither a struct nor an array.

## Topics

### Describing the struct member
- [name](https://developer.apple.com/documentation/metal/mtlstructmember/name) — The name of the struct member.
- [dataType](https://developer.apple.com/documentation/metal/mtlstructmember/datatype) — The data type of the struct member.
- [offset](https://developer.apple.com/documentation/metal/mtlstructmember/offset) — The location of this member relative to the start of its struct, in bytes.
- [argumentIndex](https://developer.apple.com/documentation/metal/mtlstructmember/argumentindex) — The index in the argument table that corresponds to the struct member.

### Obtaining struct member details
- [arrayType()](https://developer.apple.com/documentation/metal/mtlstructmember/arraytype()) — Provides a description of the underlying array when the struct member holds an array.
- [structType()](https://developer.apple.com/documentation/metal/mtlstructmember/structtype()) — Provides a description of the underlying struct when the struct member holds a struct.
- [pointerType()](https://developer.apple.com/documentation/metal/mtlstructmember/pointertype()) — Provides a description of the underlying pointer when the struct member holds a pointer.
- [textureReferenceType()](https://developer.apple.com/documentation/metal/mtlstructmember/texturereferencetype()) — Provides a description of the underlying texture when the struct member holds a texture.

### Instance Methods
- [tensorReferenceType()](https://developer.apple.com/documentation/metal/mtlstructmember/tensorreferencetype()) — Provides a description of the underlying tensor type when this struct member holds a tensor.

## See also

### Shader types
- [MTLType](https://developer.apple.com/documentation/metal/mtltype) — A description of a data type.
- [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype) — The parameter type options for GPU functions, such as shaders and compute kernels.
- [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) — A description of an array.
- [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) — A description of a structure.
- [MTLPointerType](https://developer.apple.com/documentation/metal/mtlpointertype) — A description of a pointer.
- [MTLTextureReferenceType](https://developer.apple.com/documentation/metal/mtltexturereferencetype) — A description of a texture.
