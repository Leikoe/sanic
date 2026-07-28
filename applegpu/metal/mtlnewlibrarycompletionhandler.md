# MTLNewLibraryCompletionHandler

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlnewlibrarycompletionhandler>

A completion handler signature a method calls when it finishes creating a Metal library.

## Declaration

```swift
typealias MTLNewLibraryCompletionHandler = ((any MTLLibrary)?, (any Error)?) -> Void
```

## Parameters

- **library** — An [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance if the method successfully compiles the library without any errors; otherwise `nil`.
- **error** — An error instance if the compiler generates any errors; otherwise `nil`.

## Discussion

The framework reports compiler warnings to the console. The `error` parameter doesn’t report warnings because it’s `nil` when there aren’t any compiler errors.

## See also

### Creating shader libraries
- [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()) — Creates a Metal library instance that contains the functions from your app’s default Metal library.
- [makeDefaultLibrary(bundle:)](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary(bundle:)) — Creates a Metal library instance that contains the functions in a bundle’s default Metal library.
- [makeLibrary(URL:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(url:)) — Creates a Metal library instance that contains the functions in the Metal library file at a URL.
- [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)) — Synchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) — Asynchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(stitchedDescriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:)) — Synchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(stitchedDescriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:completionhandler:)) — Asynchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)-7khmh) — Creates a Metal library instance that contains the functions in a precompiled Metal library.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)) — Creates a Metal library instance from a dispatch-data instance that contains the functions in a precompiled Metal library.
- [makeLibrary(filepath:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(filepath:)) — Creates a Metal library instance that contains the functions in the Metal library file at a file path.
