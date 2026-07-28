# MTLFunctionConstantValues

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues>

A set of constant values that specialize a graphics or compute GPU function.

## Declaration

```swift
class MTLFunctionConstantValues
```

## Overview

An [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) instance sets constant values for function constants. You declare function constants with the `[[ function_constant(index) ]]` attribute in MSL (Metal Shading Language) source code. See the [Metal Shading Language specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf) for more information.

With an [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) instance, you can set each constant value individually with an index or a name, or set multiple constant values with an index range.

You can apply a single [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) instance to multiple [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instances of any kind, such as a vertex function and a fragment function. When you create a specialized function, subsequent changes to its constant values have no effect. However, you can reset, add, or modify a constant value in your [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) instance and reuse it to create another [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance.

> **Tip:**
>  See [Using function specialization to build pipeline variants](https://developer.apple.com/documentation/metal/using-function-specialization-to-build-pipeline-variants) for a sample code project that applies function constant values.

## Topics

### Setting constant values
- [setConstantValue(_:type:index:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:index:)) — Sets a value for a function constant at a specific index.
- [setConstantValue(_:type:withName:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalue(_:type:withname:)) — Sets a value for a function constant with a specific name.
- [setConstantValues(_:type:range:)](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/setconstantvalues(_:type:range:)) — Sets values for a group of function constants within a specific index range.

### Resetting constant values
- [reset()](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues/reset()) — Deletes all previously set constant values.

## See also

### Compile-time variant functions
- [MTLFunctionConstant](https://developer.apple.com/documentation/metal/mtlfunctionconstant) — A constant that specializes the behavior of a shader.
