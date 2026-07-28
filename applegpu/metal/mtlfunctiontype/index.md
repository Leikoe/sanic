# MTLFunctionType

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctiontype>

The type of a top-level Metal Shading Language (MSL) function.

## Declaration

```swift
enum MTLFunctionType
```

## Topics

### Function types
- [MTLFunctionType.vertex](https://developer.apple.com/documentation/metal/mtlfunctiontype/vertex) — A vertex function you can use in a render pipeline state object.
- [MTLFunctionType.fragment](https://developer.apple.com/documentation/metal/mtlfunctiontype/fragment) — A fragment function you can use in a render pipeline state object.
- [MTLFunctionType.kernel](https://developer.apple.com/documentation/metal/mtlfunctiontype/kernel) — A kernel you can use in a compute pipeline state object.
- [MTLFunctionType.intersection](https://developer.apple.com/documentation/metal/mtlfunctiontype/intersection) — A function you can use in an intersection function table.
- [MTLFunctionType.visible](https://developer.apple.com/documentation/metal/mtlfunctiontype/visible) — A function you can use in a visible function table.

### Enumeration Cases
- [MTLFunctionType.mesh](https://developer.apple.com/documentation/metal/mtlfunctiontype/mesh)
- [MTLFunctionType.object](https://developer.apple.com/documentation/metal/mtlfunctiontype/object)

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlfunctiontype/init(rawvalue:))

## See also

### Identifying shader functions
- [device](https://developer.apple.com/documentation/metal/mtlfunction/device) — The device object that created the shader function.
- [label](https://developer.apple.com/documentation/metal/mtlfunction/label) — A string that identifies the shader function.
- [functionType](https://developer.apple.com/documentation/metal/mtlfunction/functiontype) — The shader function’s type.
- [name](https://developer.apple.com/documentation/metal/mtlfunction/name) — The function’s name.
- [options](https://developer.apple.com/documentation/metal/mtlfunction/options) — The options that Metal used to compile this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
