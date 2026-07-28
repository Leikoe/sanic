# MTLDynamicLibraryError

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct>

Errors when compiling dynamic libraries.

## Declaration

```swift
struct MTLDynamicLibraryError
```

## Topics

### Error codes
- [none](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/none) — An error code that represents the absence of any problems.
- [invalidFile](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/invalidfile) — An error code that indicates an app is using an invalid reference to a library file, typically related to a URL.
- [compilationFailure](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/compilationfailure) — An error code that indicates Metal couldn’t compile a dynamic library.
- [unresolvedInstallName](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/unresolvedinstallname) — An error code that indicates Metal couldn’t resolve the installation name for a new dynamic library.
- [dependencyLoadFailure](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/dependencyloadfailure) — An error code that indicates a dynamic library couldn’t link to other dynamic libraries.
- [unsupported](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/unsupported) — An error code that indicates the GPU device doesn’t support dynamic libraries.
- [MTLDynamicLibraryError.Code](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/code) — Error codes that Metal can generate when creating dynamic libraries.

### Error domain
- [errorDomain](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct/errordomain) — The current dynamic library error domain.
- [MTLDynamicLibraryDomain](https://developer.apple.com/documentation/metal/mtldynamiclibrarydomain) — The domain for Metal dynamic library errors.

## See also

### Structures
- [MTLTensorError](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct)
- [MTLBinaryArchiveError](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct) — An error that occurred when creating a binary shader archive.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLComponentTransform](https://developer.apple.com/documentation/metal/mtlcomponenttransform)
- [MTLCounterSampleBufferError](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct) — The error codes that indicate why a GPU driver can’t create a counter sample buffer.
- [MTLIOError](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct) — The categories of errors for creating an input/output file handle.
- [MTLPackedFloatQuaternion](https://developer.apple.com/documentation/metal/mtlpackedfloatquaternion)
- [MTLStitchedLibraryOptions](https://developer.apple.com/documentation/metal/mtlstitchedlibraryoptions)
- [NSDeviceCertification](https://developer.apple.com/documentation/metal/nsdevicecertification)
- [NSProcessPerformanceProfile](https://developer.apple.com/documentation/metal/nsprocessperformanceprofile) — A value describing the device’s performance profile.
