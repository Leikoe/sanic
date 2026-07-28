# MTLLibraryOptimizationLevel

*Enumeration · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel>

The optimization options for the Metal compiler.

## Declaration

```swift
enum MTLLibraryOptimizationLevel
```

## Topics

### Optimization options
- [MTLLibraryOptimizationLevel.default](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel/default) — An optimization option for the Metal compiler that prioritizes runtime performance.
- [MTLLibraryOptimizationLevel.size](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel/size) — An optimization option for the Metal compiler that prioritizes minimizing the size of its output binaries, which may also reduce compile time.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel/init(rawvalue:))

## See also

### Shader library management
- [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) — A collection of Metal shader functions.
- [MTLDynamicLibrary](https://developer.apple.com/documentation/metal/mtldynamiclibrary) — A dynamically linkable representation of compiled shader code for a specific Metal device object.
- [MTLBinaryArchive](https://developer.apple.com/documentation/metal/mtlbinaryarchive) — A container for pipeline state descriptors and their associated compiled shader code.
- [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) — Compilation settings for a Metal shader library.
- [MTLLibraryType](https://developer.apple.com/documentation/metal/mtllibrarytype) — A set of options for Metal library types.
- [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) — Metal shading language versions.
- [MTLCompileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompilesymbolvisibility)
