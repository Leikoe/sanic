# makeDefaultLibrary()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()>

Creates a Metal library instance that contains the functions from your app’s default Metal library.

## Declaration

```swift
func makeDefaultLibrary() -> (any MTLLibrary)?
```

## Return Value

A new [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance if the method completes successfully; otherwise `nil`.

## Discussion

Xcode compiles all the Metal source files (ending in `.metal`) in an Xcode project into a single default library.

## See also

### Creating shader libraries
- [makeDefaultLibrary(bundle:)](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary(bundle:)) — Creates a Metal library instance that contains the functions in a bundle’s default Metal library.
- [makeLibrary(URL:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(url:)) — Creates a Metal library instance that contains the functions in the Metal library file at a URL.
- [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)) — Synchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(source:options:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) — Asynchronously creates a Metal library instance by compiling the functions in a source string.
- [makeLibrary(stitchedDescriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:)) — Synchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(stitchedDescriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(stitcheddescriptor:completionhandler:)) — Asynchronously creates a Metal library from the function stitching graphs in a descriptor.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)-7khmh) — Creates a Metal library instance that contains the functions in a precompiled Metal library.
- [makeLibrary(data:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(data:)) — Creates a Metal library instance from a dispatch-data instance that contains the functions in a precompiled Metal library.
- [MTLNewLibraryCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewlibrarycompletionhandler) — A completion handler signature a method calls when it finishes creating a Metal library.
- [makeLibrary(filepath:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(filepath:)) — Creates a Metal library instance that contains the functions in the Metal library file at a file path.
