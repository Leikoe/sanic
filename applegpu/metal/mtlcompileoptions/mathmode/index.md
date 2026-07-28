# mathMode

*Instance Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode>

An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.

## Declaration

```swift
var mathMode: MTLMathMode { get set }
```

## Discussion

This property replaces the [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled) property.

If [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled) is `true`, the system sets [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) to [MTLMathMode.fast](https://developer.apple.com/documentation/metal/mtlmathmode/fast) and [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) to [MTLMathFloatingPointFunctions.fast](https://developer.apple.com/documentation/metal/mtlmathfloatingpointfunctions/fast).

If [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled) is `false`, the system sets [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) to [MTLMathMode.safe](https://developer.apple.com/documentation/metal/mtlmathmode/safe) and [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) to [MTLMathFloatingPointFunctions.precise](https://developer.apple.com/documentation/metal/mtlmathfloatingpointfunctions/precise).

Subsequent calls to [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) or [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) set the variables directly.

## Topics

### Supporting types
- [MTLMathMode](https://developer.apple.com/documentation/metal/mtlmathmode) — An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.

## See also

### Configuring the compiler options
- [enableLogging](https://developer.apple.com/documentation/metal/mtlcompileoptions/enablelogging) — A Boolean value that enables shader logging.
- [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) — The FP32 math functions Metal uses.
- [preserveInvariance](https://developer.apple.com/documentation/metal/mtlcompileoptions/preserveinvariance) — A Boolean value that indicates whether the compiler compiles vertex shaders conservatively to generate consistent position calculations.
- [languageVersion](https://developer.apple.com/documentation/metal/mtlcompileoptions/languageversion) — The language version for interpreting the library source code.
- [preprocessorMacros](https://developer.apple.com/documentation/metal/mtlcompileoptions/preprocessormacros) — A list of preprocessor macros to apply when compiling the library source.
- [optimizationLevel](https://developer.apple.com/documentation/metal/mtlcompileoptions/optimizationlevel) — An option that tells the compiler what to prioritize when it compiles Metal shader code.
- [libraries](https://developer.apple.com/documentation/metal/mtlcompileoptions/libraries) — An array of dynamic libraries the Metal compiler links against.
- [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled) — A Boolean value that indicates whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.
