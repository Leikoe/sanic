# label

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunction/label>

A string that identifies the shader function.

## Declaration

```swift
var label: String? { get set }
```

## Discussion

Object and command labels are useful identifiers at runtime or when profiling and debugging your app using any Metal tool. See [Naming resources and commands](https://developer.apple.com/documentation/Xcode/Naming-resources-and-commands).

## See also

### Identifying shader functions
- [device](https://developer.apple.com/documentation/metal/mtlfunction/device) — The device object that created the shader function.
- [functionType](https://developer.apple.com/documentation/metal/mtlfunction/functiontype) — The shader function’s type.
- [name](https://developer.apple.com/documentation/metal/mtlfunction/name) — The function’s name.
- [MTLFunctionType](https://developer.apple.com/documentation/metal/mtlfunctiontype) — The type of a top-level Metal Shading Language (MSL) function.
- [options](https://developer.apple.com/documentation/metal/mtlfunction/options) — The options that Metal used to compile this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
