# MTLStructType

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstructtype>

A description of a structure.

## Declaration

```swift
class MTLStructType
```

## Overview

[MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) is part of the reflection API that allows Metal framework code to query details of a struct that is passed as an argument of a Metal shading language function. Don’t create [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) instances directly; instead query the [bufferStructType](https://developer.apple.com/documentation/metal/mtlargument/bufferstructtype) property of an [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) instance, or call the [structType()](https://developer.apple.com/documentation/metal/mtlstructmember/structtype()) method for an [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) instance. To examine the details of the struct, you can recursively drill down the [members](https://developer.apple.com/documentation/metal/mtlstructtype/members) property of the [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) instance, which contains details about struct members, each of which is represented by an [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) instance.

## Topics

### Obtaining information about struct members
- [members](https://developer.apple.com/documentation/metal/mtlstructtype/members) — An array of instances that describe the fields in the struct.
- [memberByName(_:)](https://developer.apple.com/documentation/metal/mtlstructtype/memberbyname(_:)) — Provides a representation of a struct member.

## See also

### Shader types
- [MTLType](https://developer.apple.com/documentation/metal/mtltype) — A description of a data type.
- [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype) — The parameter type options for GPU functions, such as shaders and compute kernels.
- [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) — A description of an array.
- [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) — An instance that provides information about a field in a structure.
- [MTLPointerType](https://developer.apple.com/documentation/metal/mtlpointertype) — A description of a pointer.
- [MTLTextureReferenceType](https://developer.apple.com/documentation/metal/mtltexturereferencetype) — A description of a texture.
