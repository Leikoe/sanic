# access

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument/access>

The argument’s read and/or write access.

## Declaration

```swift
var access: MTLBindingAccess { get }
```

## Discussion

This property indicates the type of access qualifiers (read-only, write-only, or read-write) used in the Metal shading language code. For information on possible values, see [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess).

## See also

### Describing the argument
- [name](https://developer.apple.com/documentation/metal/mtlargument/name) — The name of the argument.
- [isActive](https://developer.apple.com/documentation/metal/mtlargument/isactive) — A Boolean that indicates whether the compiled function uses the argument.
- [index](https://developer.apple.com/documentation/metal/mtlargument/index) — The index in the argument table that corresponds to the function argument.
- [type](https://developer.apple.com/documentation/metal/mtlargument/type) — The argument’s resource type.
