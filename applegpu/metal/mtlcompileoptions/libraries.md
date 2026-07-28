# libraries

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcompileoptions/libraries>

An array of dynamic libraries the Metal compiler links against.

## Declaration

```swift
var libraries: [any MTLDynamicLibrary]? { get set }
```

## See also

### Configuring the compiler options
- [enableLogging](https://developer.apple.com/documentation/metal/mtlcompileoptions/enablelogging) — A Boolean value that enables shader logging.
- [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) — An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.
- [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) — The FP32 math functions Metal uses.
- [preserveInvariance](https://developer.apple.com/documentation/metal/mtlcompileoptions/preserveinvariance) — A Boolean value that indicates whether the compiler compiles vertex shaders conservatively to generate consistent position calculations.
- [languageVersion](https://developer.apple.com/documentation/metal/mtlcompileoptions/languageversion) — The language version for interpreting the library source code.
- [preprocessorMacros](https://developer.apple.com/documentation/metal/mtlcompileoptions/preprocessormacros) — A list of preprocessor macros to apply when compiling the library source.
- [optimizationLevel](https://developer.apple.com/documentation/metal/mtlcompileoptions/optimizationlevel) — An option that tells the compiler what to prioritize when it compiles Metal shader code.
- [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled) — A Boolean value that indicates whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.
