# MTLFunctionConstant

*Class · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionconstant>

A constant that specializes the behavior of a shader.

## Declaration

```swift
class MTLFunctionConstant
```

## Overview

Don’t create an [MTLFunctionConstant](https://developer.apple.com/documentation/metal/mtlfunctionconstant) instance directly. Instead, the list of function constants for a function by querying the `functionConstants` property of an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance.

An [MTLFunctionConstant](https://developer.apple.com/documentation/metal/mtlfunctionconstant) instance should only be obtained from a nonspecialized function created with the [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) method. You only need an [MTLFunctionConstant](https://developer.apple.com/documentation/metal/mtlfunctionconstant) instance if you don’t have sufficient information to create an [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) instance used to create a specialized function with the [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)) or [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) method.

## Topics

### Reading the function constant’s properties
- [name](https://developer.apple.com/documentation/metal/mtlfunctionconstant/name) — The name of the function constant.
- [type](https://developer.apple.com/documentation/metal/mtlfunctionconstant/type) — The data type of the function constant.
- [index](https://developer.apple.com/documentation/metal/mtlfunctionconstant/index) — The index of the function constant.
- [required](https://developer.apple.com/documentation/metal/mtlfunctionconstant/required) — A Boolean value indicating whether the function constant needs to be provided to specialize the function.

## See also

### Compile-time variant functions
- [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) — A set of constant values that specialize a graphics or compute GPU function.
