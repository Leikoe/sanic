# constantValues

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/constantvalues>

The set of constant values assigned to the function constants.

## Declaration

```swift
@NSCopying var constantValues: MTLFunctionConstantValues? { get set }
```

## Discussion

The default value is `nil`. If you are creating a function object for a specialized function, you need to provide an array of valid constant values for all required function constants.

## See also

### Specifying the function configuration
- [name](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/name) — The name of the function to fetch from the library.
- [specializedName](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/specializedname) — A new name for the created function object.
- [options](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/options) — Flags specifying how Metal should create the new function object.
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/binaryarchives) — The binary archives to search for a previously-compiled version of this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
- [MTLLinkedFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions) — A set of related functions that Metal links to when necessary to create the function instance.
