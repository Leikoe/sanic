# makeLibrary(source:options:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)>

Synchronously creates a Metal library instance by compiling the functions in a source string.

## Declaration

```swift
func makeLibrary(source: String, options: MTLCompileOptions?) throws -> any MTLLibrary
```

## Parameters

- **source** — A string that contains source code for one or more Metal functions. For information about writing source in Metal Shading Language (MSL), see the [Metal Shading Language Specification](https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf).
- **options** — An [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) instance that affects the compilation of the source code in the string.

## Return Value

A new [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

Because there’s no search path to find other functions, the source may only import the Metal default library.

## See also

### Creating shader libraries
- [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()) — Creates a Metal library instance that contains the functions from your app’s default Metal library.
- [makeDefaultLibrary(bundle:)](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary(bundle:)) — Creates a Metal library instance that contains the functions in a bundle’s default Metal library.
- [makeLibrary(URL:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(url:)) — Creates a Metal library instance that contains the functions in the Metal library file at a URL.
- [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) — Asynchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(stitchedDescriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:)) — Synchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(stitchedDescriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:completionhandler:)) — Asynchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)-7khmh) — Creates a Metal library instance that contains the functions in a precompiled Metal library.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)) — Creates a Metal library instance from a dispatch-data instance that contains the functions in a precompiled Metal library.
- [MTLNewLibraryCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewlibrarycompletionhandler) — A completion handler signature a method calls when it finishes creating a Metal library.
- [makeLibrary(filepath:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(filepath:)) — Creates a Metal library instance that contains the functions in the Metal library file at a file path.
