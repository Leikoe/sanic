# computeFunctionDescriptor

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4computepipelinedescriptor/computefunctiondescriptor>

A descriptor representing the compute pipeline’s function.

## Declaration

```swift
@NSCopying var computeFunctionDescriptor: MTL4FunctionDescriptor? { get set }
```

## Discussion

You don’t assign instances of [MTL4FunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4functiondescriptor) to this property directly, instead assign an instance of one of its subclasses, such as [MTL4LibraryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4libraryfunctiondescriptor), which represents a function from a Metal library.
