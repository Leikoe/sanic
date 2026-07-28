# dataType

*Instance Property · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentdescriptor/datatype>

The data type of the argument.

## Declaration

```swift
var dataType: MTLDataType { get set }
```

## Discussion

For a constant data argument, this value needs to match the binary format of the data stored in the buffer for that argument. For other parameter types, such as textures or samplers, specify the appropriate constant. See [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype) for possible values.

## See also

### Setting the descriptor’s properties
- [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) — The index ID of the argument.
- [access](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/access) — The access permissions of the argument.
- [arrayLength](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/arraylength) — The length of an array argument.
- [constantBlockAlignment](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/constantblockalignment) — The alignment of the constant block.
- [textureType](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/texturetype) — The texture type of a texture argument.
