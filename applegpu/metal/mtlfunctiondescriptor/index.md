# MTLFunctionDescriptor

*Class · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctiondescriptor>

A description of a function object to create.

## Declaration

```swift
class MTLFunctionDescriptor
```

## Topics

### Specifying the function configuration
- [name](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/name) — The name of the function to fetch from the library.
- [specializedName](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/specializedname) — A new name for the created function object.
- [constantValues](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/constantvalues) — The set of constant values assigned to the function constants.
- [options](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/options) — Flags specifying how Metal should create the new function object.
- [binaryArchives](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor/binaryarchives) — The binary archives to search for a previously-compiled version of this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.
- [MTLLinkedFunctions](https://developer.apple.com/documentation/metal/mtllinkedfunctions) — A set of related functions that Metal links to when necessary to create the function instance.

## See also

### Related Documentation
- [makeFunction(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:completionhandler:)) — Asynchronously creates an object representing a shader function, using the specified descriptor.
- [makeFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:)) — Synchronously creates an object representing a shader function, using the specified descriptor.

### Shader functions
- [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) — A interface that represents a public shader function in a Metal library.
- [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) — An object representing a function that you can add to a visible function table.
- [MTLVisibleFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor) — A specification of how to create a visible function table.
- [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) — A table of shader functions visible to your app that you can pass into compute commands to customize the behavior of a shader.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) — A table of intersection functions that Metal calls to perform ray-tracing intersection tests.
