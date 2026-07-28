# dataType

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstructmember/datatype>

The data type of the struct member.

## Declaration

```swift
var dataType: MTLDataType { get }
```

## Discussion

For information on possible values, see [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype). If the value is [MTLDataType.array](https://developer.apple.com/documentation/metal/mtldatatype/array), then the [arrayType()](https://developer.apple.com/documentation/metal/mtlstructmember/arraytype()) method returns an object that describes the underlying array. If the value is [MTLDataType.struct](https://developer.apple.com/documentation/metal/mtldatatype/struct), then the [structType()](https://developer.apple.com/documentation/metal/mtlstructmember/structtype()) method returns an object that describes the underlying struct.

## See also

### Describing the struct member
- [name](https://developer.apple.com/documentation/metal/mtlstructmember/name) — The name of the struct member.
- [offset](https://developer.apple.com/documentation/metal/mtlstructmember/offset) — The location of this member relative to the start of its struct, in bytes.
- [argumentIndex](https://developer.apple.com/documentation/metal/mtlstructmember/argumentindex) — The index in the argument table that corresponds to the struct member.
