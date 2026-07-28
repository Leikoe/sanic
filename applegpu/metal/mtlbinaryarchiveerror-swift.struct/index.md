# MTLBinaryArchiveError

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct>

An error that occurred when creating a binary shader archive.

## Declaration

```swift
struct MTLBinaryArchiveError
```

## Topics

### Error codes
- [none](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/none) — An error code that represents the absence of any problems.
- [invalidFile](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/invalidfile) — An error code that indicates an app is using an invalid reference to an archive file, typically related to a URL.
- [compilationFailure](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/compilationfailure) — An error code that indicates the archive’s inability to compile its contents, typically when serializing it to a URL.
- [unexpectedElement](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/unexpectedelement) — An error code that indicates a problem with a configuration, typically in a descriptor or an archive’s inability to add linked functions.
- [internalError](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/internalerror) — An error code that indicates the Metal framework has an internal problem.
- [MTLBinaryArchiveError.Code](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/code) — Error codes when creating binary archives of compiled shader code.

### Error domain
- [errorDomain](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/errordomain) — The current binary archive error domain.
- [MTLBinaryArchiveDomain](https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain) — The domain for Metal binary archive errors.

## See also

### Structures
- [MTLTensorError](https://developer.apple.com/documentation/metal/mtltensorerror-swift.struct)
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
- [MTLComponentTransform](https://developer.apple.com/documentation/metal/mtlcomponenttransform)
- [MTLCounterSampleBufferError](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror-swift.struct) — The error codes that indicate why a GPU driver can’t create a counter sample buffer.
- [MTLDynamicLibraryError](https://developer.apple.com/documentation/metal/mtldynamiclibraryerror-swift.struct) — Errors when compiling dynamic libraries.
- [MTLIOError](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct) — The categories of errors for creating an input/output file handle.
- [MTLPackedFloatQuaternion](https://developer.apple.com/documentation/metal/mtlpackedfloatquaternion)
- [MTLStitchedLibraryOptions](https://developer.apple.com/documentation/metal/mtlstitchedlibraryoptions)
- [NSDeviceCertification](https://developer.apple.com/documentation/metal/nsdevicecertification)
- [NSProcessPerformanceProfile](https://developer.apple.com/documentation/metal/nsprocessperformanceprofile) — A value describing the device’s performance profile.
