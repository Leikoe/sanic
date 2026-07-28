# binaryArchives

*Instance Property · iOS 15.0, iPadOS 15.0, Mac Catalyst 15.0, macOS 12.0, tvOS 15.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/binaryarchives>

The binary archives to search for a previously-compiled version of this function.

## Declaration

```swift
var binaryArchives: [any MTLBinaryArchive]? { get set }
```

## Discussion

If you specify an archive that includes a fully compiled version of this function, Metal uses the compiled version rather than creating a new one.

## See also

### Specifying the function configuration
- [name](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/name) — The name of the function to fetch from the library.
- [specializedName](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/specializedname) — A new name for the created function object.
- [constantValues](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/constantvalues) — The set of constant values assigned to the function constants.
- [options](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/options) — Flags specifying how Metal should create the new function object.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
- [MTLLinkedFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions) — A set of related functions that Metal links to when necessary to create the function instance.
