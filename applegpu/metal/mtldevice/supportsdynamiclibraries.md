# supportsDynamicLibraries

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportsdynamiclibraries>

A Boolean value that indicates whether the GPU device can create and use dynamic libraries in compute pipelines.

## Declaration

```swift
var supportsDynamicLibraries: Bool { get }
```

## See also

### Creating dynamic shader libraries
- [supportsRenderDynamicLibraries](https://developer.apple.com/documentation/metal/mtldevice/supportsrenderdynamiclibraries) — A Boolean value that indicates whether the GPU device can create and use dynamic libraries in render pipelines.
- [makeDynamicLibrary(library:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(library:)) — Creates a Metal dynamic library instance from a Metal library instance.
- [makeDynamicLibrary(url:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(url:)) — Creates a Metal dynamic library instance that contains the functions in the Metal library file at a URL.
- [MTLDynamicLibraryError.Code](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/code) — Error codes that Metal can generate when creating dynamic libraries.
- [MTLDynamicLibraryDomain](https://developer.apple.com/documentation/metal/mtldynamiclibrarydomain) — The domain for Metal dynamic library errors.
