# unsupported

*Type Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/unsupported>

Metal couldn’t support the requested action.

## Declaration

```swift
static var unsupported: MTLLibraryError.Code { get }
```

## Discussion

For example, the requested library file has improper formatting, or the requested library isn’t accessible.

## See also

### Errors
- [internal](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/internal) — The action caused an internal error.
- [compileFailure](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/compilefailure) — The library or function failed to compile.
- [compileWarning](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/compilewarning) — The library or function compiled successfully but generated warnings.
- [fileNotFound](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/filenotfound) — Metal couldn’t find the Metal source file.
- [functionNotFound](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/functionnotfound) — Metal couldn’t find the specified Metal function.
