# makeFunction(descriptor:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:)>

Synchronously creates an object representing a shader function, using the specified descriptor.

## Declaration

```swift
func makeFunction(descriptor: MTLFunctionDescriptor) throws -> any MTLFunction
```

## Parameters

- **descriptor** — The description of the function object to create.

## Return Value

A new [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance if the method finds the function in the library; otherwise Swift throws an error and Objective-C returns `nil`.

## See also

### Creating shader function instances
- [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) — Creates an instance that represents a shader function in the library.
- [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) — Asynchronously creates a specialized shader function.
- [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)) — Synchronously creates a specialized shader function.
- [makeFunction(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:completionhandler:)) — Asynchronously creates an object representing a shader function, using the specified descriptor.
