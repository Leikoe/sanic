# isActive

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument/isactive>

A Boolean that indicates whether the compiled function uses the argument.

## Declaration

```swift
var isActive: Bool { get }
```

## Discussion

When you create the [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) object, Metal statically determines whether the function uses the argument. If [true](https://developer.apple.com/documentation/Swift/true), you need to provide a value for this argument when you encode a command that calls this function. If [false](https://developer.apple.com/documentation/Swift/false), the function doesn’t use the argument, and you can ignore it.

## See also

### Describing the argument
- [name](https://developer.apple.com/documentation/metal/mtlargument/name) — The name of the argument.
- [index](https://developer.apple.com/documentation/metal/mtlargument/index) — The index in the argument table that corresponds to the function argument.
- [type](https://developer.apple.com/documentation/metal/mtlargument/type) — The argument’s resource type.
- [access](https://developer.apple.com/documentation/metal/mtlargument/access) — The argument’s read and/or write access.
