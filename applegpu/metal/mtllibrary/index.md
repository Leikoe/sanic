# MTLLibrary

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibrary>

A collection of Metal shader functions.

## Declaration

```swift
protocol MTLLibrary : NSObjectProtocol, Sendable
```

## Overview

An [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance contains Metal shading language source code compiled during an app’s build process or at runtime from a text string.

Don’t implement this protocol yourself; instead, use the library creation methods provided by the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) protocol. To create an [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) from a precompiled Metal library binary, call one of these [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods:

- [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary())

- [makeLibrary(filepath:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(filepath:))

- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:))

To create an [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) by compiling source code at runtime, call one of these [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods:

- [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:))

- [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:))

## Topics

### Querying basic library attributes
- [installName](https://developer.apple.com/documentation/metal/mtllibrary/installname) — The installation name for a dynamic library.
- [type](https://developer.apple.com/documentation/metal/mtllibrary/type) — The library’s basic type.

### Querying library contents
- [functionNames](https://developer.apple.com/documentation/metal/mtllibrary/functionnames) — The names of all public functions in the library.

### Creating shader function instances
- [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) — Creates an instance that represents a shader function in the library.
- [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) — Asynchronously creates a specialized shader function.
- [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)) — Synchronously creates a specialized shader function.
- [makeFunction(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:completionhandler:)) — Asynchronously creates an object representing a shader function, using the specified descriptor.
- [makeFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:)) — Synchronously creates an object representing a shader function, using the specified descriptor.

### Creating intersection function instances
- [makeIntersectionFunction(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makeintersectionfunction(descriptor:completionhandler:)) — Asynchronously creates an object representing a ray-tracing intersection function, using the specified descriptor.
- [makeIntersectionFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtllibrary/makeintersectionfunction(descriptor:)) — Synchronously creates an object representing a ray-tracing intersection function, using the specified descriptor.

### Identifying the library
- [device](https://developer.apple.com/documentation/metal/mtllibrary/device) — The Metal device object that created the library.
- [label](https://developer.apple.com/documentation/metal/mtllibrary/label) — A string that identifies the library.

### Instance Methods
- [reflection(functionName:)](https://developer.apple.com/documentation/metal/mtllibrary/reflection(functionname:)) — Retrieves reflection information for a function in the library.

## See also

### Shader library management
- [MTLDynamicLibrary](https://developer.apple.com/documentation/metal/mtldynamiclibrary) — A dynamically linkable representation of compiled shader code for a specific Metal device object.
- [MTLBinaryArchive](https://developer.apple.com/documentation/metal/mtlbinaryarchive) — A container for pipeline state descriptors and their associated compiled shader code.
- [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) — Compilation settings for a Metal shader library.
- [MTLLibraryType](https://developer.apple.com/documentation/metal/mtllibrarytype) — A set of options for Metal library types.
- [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) — Metal shading language versions.
- [MTLCompileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompilesymbolvisibility)
- [MTLLibraryOptimizationLevel](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel) — The optimization options for the Metal compiler.
