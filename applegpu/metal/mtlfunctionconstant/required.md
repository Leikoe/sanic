# required

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionconstant/required>

A Boolean value indicating whether the function constant needs to be provided to specialize the function.

## Declaration

```swift
var required: Bool { get }
```

## Discussion

This value is [true](https://developer.apple.com/documentation/Swift/true) if a constant value needs to be provided for the function constant. A function constant is optional only if it is referenced in a call to the built-in `is_function_constant_defined(name)` function.

Refer to the [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364) for more information.

## See also

### Reading the function constant’s properties
- [name](https://developer.apple.com/documentation/metal/mtlfunctionconstant/name) — The name of the function constant.
- [type](https://developer.apple.com/documentation/metal/mtlfunctionconstant/type) — The data type of the function constant.
- [index](https://developer.apple.com/documentation/metal/mtlfunctionconstant/index) — The index of the function constant.
