# MTLCompileOptions

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcompileoptions>

Compilation settings for a Metal shader library.

## Declaration

```swift
class MTLCompileOptions
```

## Overview

You can configure the Metal compiler’s options by setting any or all of an [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) instance’s properties, including the following:

- Target previous OS releases by assigning the [languageVersion](https://developer.apple.com/documentation/metal/mtlcompileoptions/languageversion) property to an [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) case.

- Set preprocessor macros for the Metal compiler by assigning a dictionary to the [preprocessorMacros](https://developer.apple.com/documentation/metal/mtlcompileoptions/preprocessormacros) property.

- Choose what the Metal compiler’s optimizer prioritizes by setting the [optimizationLevel](https://developer.apple.com/documentation/metal/mtlcompileoptions/optimizationlevel) property to an [MTLLibraryOptimizationLevel](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel) case.

- Allow the compiler to optimize for floating-point arithmetic that may violate the IEEE 754 standard by setting [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) to [MTLMathMode.fast](https://developer.apple.com/documentation/metal/mtlmathmode/fast).

You can compile a library with your compile options instance by calling an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)) or [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) method.

## Topics

### Configuring the compiler options
- [enableLogging](https://developer.apple.com/documentation/metal/mtlcompileoptions/enablelogging) — A Boolean value that enables shader logging.
- [mathMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathmode) — An indication of whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.
- [mathFloatingPointFunctions](https://developer.apple.com/documentation/metal/mtlcompileoptions/mathfloatingpointfunctions) — The FP32 math functions Metal uses.
- [preserveInvariance](https://developer.apple.com/documentation/metal/mtlcompileoptions/preserveinvariance) — A Boolean value that indicates whether the compiler compiles vertex shaders conservatively to generate consistent position calculations.
- [languageVersion](https://developer.apple.com/documentation/metal/mtlcompileoptions/languageversion) — The language version for interpreting the library source code.
- [preprocessorMacros](https://developer.apple.com/documentation/metal/mtlcompileoptions/preprocessormacros) — A list of preprocessor macros to apply when compiling the library source.
- [optimizationLevel](https://developer.apple.com/documentation/metal/mtlcompileoptions/optimizationlevel) — An option that tells the compiler what to prioritize when it compiles Metal shader code.
- [libraries](https://developer.apple.com/documentation/metal/mtlcompileoptions/libraries) — An array of dynamic libraries the Metal compiler links against.
- [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/fastmathenabled) — A Boolean value that indicates whether the compiler can perform optimizations for floating-point arithmetic that may violate the IEEE 754 standard.

### Configuring the library output options
- [libraryType](https://developer.apple.com/documentation/metal/mtlcompileoptions/librarytype) — The kind of library to create.
- [installName](https://developer.apple.com/documentation/metal/mtlcompileoptions/installname) — For a dynamic library, the name to use when installing the library.

### Instance Properties
- [allowReferencingUndefinedSymbols](https://developer.apple.com/documentation/metal/mtlcompileoptions/allowreferencingundefinedsymbols)
- [compileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompileoptions/compilesymbolvisibility)
- [floatingPointConversionRoundingMode](https://developer.apple.com/documentation/metal/mtlcompileoptions/floatingpointconversionroundingmode)
- [maxTotalThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcompileoptions/maxtotalthreadsperthreadgroup)
- [requiredThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtlcompileoptions/requiredthreadsperthreadgroup)

## See also

### Shader library management
- [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) — A collection of Metal shader functions.
- [MTLDynamicLibrary](https://developer.apple.com/documentation/metal/mtldynamiclibrary) — A dynamically linkable representation of compiled shader code for a specific Metal device object.
- [MTLBinaryArchive](https://developer.apple.com/documentation/metal/mtlbinaryarchive) — A container for pipeline state descriptors and their associated compiled shader code.
- [MTLLibraryType](https://developer.apple.com/documentation/metal/mtllibrarytype) — A set of options for Metal library types.
- [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) — Metal shading language versions.
- [MTLCompileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompilesymbolvisibility)
- [MTLLibraryOptimizationLevel](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel) — The optimization options for the Metal compiler.
