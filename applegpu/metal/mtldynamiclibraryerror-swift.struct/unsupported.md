# unsupported

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/unsupported>

An error code that indicates the GPU device doesn’t support dynamic libraries.

## Declaration

```swift
static var unsupported: MTLDynamicLibraryError.Code { get }
```

## See also

### Error codes
- [none](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/none) — An error code that represents the absence of any problems.
- [invalidFile](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/invalidfile) — An error code that indicates an app is using an invalid reference to a library file, typically related to a URL.
- [compilationFailure](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/compilationfailure) — An error code that indicates Metal couldn’t compile a dynamic library.
- [unresolvedInstallName](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/unresolvedinstallname) — An error code that indicates Metal couldn’t resolve the installation name for a new dynamic library.
- [dependencyLoadFailure](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/dependencyloadfailure) — An error code that indicates a dynamic library couldn’t link to other dynamic libraries.
- [MTLDynamicLibraryError.Code](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/code) — Error codes that Metal can generate when creating dynamic libraries.
