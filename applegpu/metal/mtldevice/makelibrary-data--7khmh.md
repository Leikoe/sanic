# makeLibrary(data:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 8.0, macOS 10.11, tvOS 8.0, visionOS*

<https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)-7khmh>

Creates a Metal library instance that contains the functions in a precompiled Metal library.

## Declaration

```swift
func makeLibrary(data: DispatchData) throws -> any MTLLibrary
```

## Parameters

- **data** — The data from a precompiled Metal library. For more information, see [Building a shader library by precompiling source files](https://developer.apple.com/documentation/metal/building-a-shader-library-by-precompiling-source-files).

## Return Value

A new [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

Use this method if your application manages its own archiving system for libraries — for example, if your app uses a single file that contains several libraries.

> **Note:**
>  This is a Swift default implementation for the [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)) method.

## See also

### Creating shader libraries
- [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()) — Creates a Metal library instance that contains the functions from your app’s default Metal library.
- [makeDefaultLibrary(bundle:)](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary(bundle:)) — Creates a Metal library instance that contains the functions in a bundle’s default Metal library.
- [makeLibrary(URL:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(url:)) — Creates a Metal library instance that contains the functions in the Metal library file at a URL.
- [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)) — Synchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) — Asynchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(stitchedDescriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:)) — Synchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(stitchedDescriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:completionhandler:)) — Asynchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)) — Creates a Metal library instance from a dispatch-data instance that contains the functions in a precompiled Metal library.
- [MTLNewLibraryCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewlibrarycompletionhandler) — A completion handler signature a method calls when it finishes creating a Metal library.
- [makeLibrary(filepath:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(filepath:)) — Creates a Metal library instance that contains the functions in the Metal library file at a file path.
