# fastMathEnabled

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled>

A Boolean value that indicates whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.

## Declaration

```swift
var fastMathEnabled: Bool { get set }
```

## Discussion

The default value is [true](https://developer.apple.com/documentation/Swift/true). A [true](https://developer.apple.com/documentation/Swift/true) value also enables the high-precision variant of math functions for single-precision floating-point scalar and vector types.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Configuring the compiler options
- [enableLogging](https://developer.apple.com/documentation/metal/mtlcompileoptions/enablelogging) — A Boolean value that enables shader logging.
- [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) — An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.
- [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) — The FP32 math functions Metal uses.
- [preserveInvariance](https://developer.apple.com/documentation/metal/mtlcompileoptions/preserveinvariance) — A Boolean value that indicates whether the compiler compiles vertex shaders conservatively to generate consistent position calculations.
- [languageVersion](https://developer.apple.com/documentation/metal/mtlcompileoptions/languageversion) — The language version for interpreting the library source code.
- [preprocessorMacros](https://developer.apple.com/documentation/metal/mtlcompileoptions/preprocessormacros) — A list of preprocessor macros to apply when compiling the library source.
- [optimizationLevel](https://developer.apple.com/documentation/metal/mtlcompileoptions/optimizationlevel) — An option that tells the compiler what to prioritize when it compiles Metal shader code.
- [libraries](https://developer.apple.com/documentation/metal/mtlcompileoptions/libraries) — An array of dynamic libraries the Metal compiler links against.
