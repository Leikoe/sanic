# makeDynamicLibrary(library:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(library:)>

Creates a Metal dynamic library instance from a Metal library instance.

## Declaration

```swift
func makeDynamicLibrary(library: any MTLLibrary) throws -> any MTLDynamicLibrary
```

## Parameters

- **library** — An [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance.

## Return Value

A new [MTLDynamicLibrary](https://developer.apple.com/documentation/metal/mtldynamiclibrary) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## See also

### Creating dynamic shader libraries
- [supportsDynamicLibraries](https://developer.apple.com/documentation/metal/mtldevice/supportsdynamiclibraries) — A Boolean value that indicates whether the GPU device can create and use dynamic libraries in compute pipelines.
- [supportsRenderDynamicLibraries](https://developer.apple.com/documentation/metal/mtldevice/supportsrenderdynamiclibraries) — A Boolean value that indicates whether the GPU device can create and use dynamic libraries in render pipelines.
- [makeDynamicLibrary(url:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(url:)) — Creates a Metal dynamic library instance that contains the functions in the Metal library file at a URL.
- [MTLDynamicLibraryError.Code](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/code) — Error codes that Metal can generate when creating dynamic libraries.
- [MTLDynamicLibraryDomain](https://developer.apple.com/documentation/metal/mtldynamiclibrarydomain) — The domain for Metal dynamic library errors.
