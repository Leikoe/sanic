# MTLVisibleFunctionTable

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable>

A table of shader functions visible to your app that you can pass into compute commands to customize the behavior of a shader.

## Declaration

```swift
protocol MTLVisibleFunctionTable : MTLResource
```

## Topics

### Setting a table entry
- [setFunction(_:index:)](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/setfunction(_:index:)) — Sets a table entry to point to a callable function.
- [setFunctions(_:range:)](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/setfunctions(_:range:)) — Sets a range of table entries to point to an array of callable functions.

### Instance Properties
- [gpuResourceID](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable/gpuresourceid)

## See also

### Shader functions
- [MTLFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor) — A description of a function object to create.
- [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) — A interface that represents a public shader function in a Metal library.
- [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) — An object representing a function that you can add to a visible function table.
- [MTLVisibleFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor) — A specification of how to create a visible function table.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) — A table of intersection functions that Metal calls to perform ray-tracing intersection tests.
