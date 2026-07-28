# device

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunction/device>

The device object that created the shader function.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

You can only use this function object with this [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice).

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Identifying shader functions
- [label](https://developer.apple.com/documentation/metal/mtlfunction/label) — A string that identifies the shader function.
- [functionType](https://developer.apple.com/documentation/metal/mtlfunction/functiontype) — The shader function’s type.
- [name](https://developer.apple.com/documentation/metal/mtlfunction/name) — The function’s name.
- [MTLFunctionType](https://developer.apple.com/documentation/metal/mtlfunctiontype) — The type of a top-level Metal Shading Language (MSL) function.
- [options](https://developer.apple.com/documentation/metal/mtlfunction/options) — The options that Metal used to compile this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
