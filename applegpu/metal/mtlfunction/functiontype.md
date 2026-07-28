# functionType

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunction/functiontype>

The shader function’s type.

## Declaration

```swift
var functionType: MTLFunctionType { get }
```

## Discussion

A function’s type determines what kind of pipeline state objects you can create from it and whether you can use it as a callable function in a function table.

## See also

### Identifying shader functions
- [device](https://developer.apple.com/documentation/metal/mtlfunction/device) — The device object that created the shader function.
- [label](https://developer.apple.com/documentation/metal/mtlfunction/label) — A string that identifies the shader function.
- [name](https://developer.apple.com/documentation/metal/mtlfunction/name) — The function’s name.
- [MTLFunctionType](https://developer.apple.com/documentation/metal/mtlfunctiontype) — The type of a top-level Metal Shading Language (MSL) function.
- [options](https://developer.apple.com/documentation/metal/mtlfunction/options) — The options that Metal used to compile this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
