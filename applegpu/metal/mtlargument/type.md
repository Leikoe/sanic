# type

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument/type>

The argument’s resource type.

## Declaration

```swift
var type: MTLArgumentType { get }
```

## Discussion

This property indicates which type of resource is used (buffer, texture, sampler, or threadgroup memory) in the shading language code. For information on possible values, see [MTLArgumentType](https://developer.apple.com/documentation/metal/mtlargumenttype).

## See also

### Describing the argument
- [name](https://developer.apple.com/documentation/metal/mtlargument/name) — The name of the argument.
- [isActive](https://developer.apple.com/documentation/metal/mtlargument/isactive) — A Boolean that indicates whether the compiled function uses the argument.
- [index](https://developer.apple.com/documentation/metal/mtlargument/index) — The index in the argument table that corresponds to the function argument.
- [access](https://developer.apple.com/documentation/metal/mtlargument/access) — The argument’s read and/or write access.
