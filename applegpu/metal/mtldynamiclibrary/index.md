# MTLDynamicLibrary

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldynamiclibrary>

A dynamically linkable representation of compiled shader code for a specific Metal device object.

## Declaration

```swift
protocol MTLDynamicLibrary : NSObjectProtocol, Sendable
```

## Topics

### Identifying the library
- [device](https://developer.apple.com/documentation/metal/mtldynamiclibrary/device) — The Metal device object that created the dynamic library.
- [installName](https://developer.apple.com/documentation/metal/mtldynamiclibrary/installname) — A file path for this dynamic library.
- [label](https://developer.apple.com/documentation/metal/mtldynamiclibrary/label) — A string that identifies the library.

### Saving a dynamic library to a file
- [serialize(to:)](https://developer.apple.com/documentation/metal/mtldynamiclibrary/serialize(to:)) — Writes the contents of the dynamic library to a file.

## See also

### Shader library management
- [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) — A collection of Metal shader functions.
- [MTLBinaryArchive](https://developer.apple.com/documentation/metal/mtlbinaryarchive) — A container for pipeline state descriptors and their associated compiled shader code.
- [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) — Compilation settings for a Metal shader library.
- [MTLLibraryType](https://developer.apple.com/documentation/metal/mtllibrarytype) — A set of options for Metal library types.
- [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) — Metal shading language versions.
- [MTLCompileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompilesymbolvisibility)
- [MTLLibraryOptimizationLevel](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel) — The optimization options for the Metal compiler.
