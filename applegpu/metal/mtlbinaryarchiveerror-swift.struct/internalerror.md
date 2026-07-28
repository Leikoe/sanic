# internalError

*Type Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/internalerror>

An error code that indicates the Metal framework has an internal problem.

## Declaration

```swift
static var internalError: MTLBinaryArchiveError.Code { get }
```

## Discussion

You can report the scenario that generated this error code with [Feedback Assistant](https://feedbackassistant.apple.com).

## See also

### Error codes
- [none](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/none) — An error code that represents the absence of any problems.
- [invalidFile](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/invalidfile) — An error code that indicates an app is using an invalid reference to an archive file, typically related to a URL.
- [compilationFailure](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/compilationfailure) — An error code that indicates the archive’s inability to compile its contents, typically when serializing it to a URL.
- [unexpectedElement](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/unexpectedelement) — An error code that indicates a problem with a configuration, typically in a descriptor or an archive’s inability to add linked functions.
- [MTLBinaryArchiveError.Code](https://developer.apple.com/documentation/metal/mtlbinaryarchiveerror-swift.struct/code) — Error codes when creating binary archives of compiled shader code.
