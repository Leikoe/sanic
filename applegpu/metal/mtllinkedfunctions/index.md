# MTLLinkedFunctions

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllinkedfunctions>

A set of related functions that Metal links to when necessary to create the function instance.

## Declaration

```swift
class MTLLinkedFunctions
```

## Overview

When you create a Metal function instance using an [MTLFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor), you specify additional functions that Metal needs to link to when it compiles and links the underlying shader code. Most often, you need to do this if your shader takes a visible function table as one or more of its arguments. For Metal to create the [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance, it needs a complete list of functions that your shader can call so that it can resolve any dependencies and generate the correct code to run on the GPU.

## Topics

### Specifying related functions
- [functions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/functions) — An array of function objects to link to the new function.
- [binaryFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/binaryfunctions) — An array of function objects already compiled to a binary representation to link.
- [groups](https://developer.apple.com/documentation/metal/mtllinkedfunctions/groups) — An optional list of groups specifying which functions your shader can call at each call site.
- [privateFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions/privatefunctions) — An array of function objects to link to the new function, without exporting the functions publicly.

## See also

### Specifying the function configuration
- [name](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/name) — The name of the function to fetch from the library.
- [specializedName](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/specializedname) — A new name for the created function object.
- [constantValues](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/constantvalues) — The set of constant values assigned to the function constants.
- [options](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/options) — Flags specifying how Metal should create the new function object.
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/binaryarchives) — The binary archives to search for a previously-compiled version of this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
