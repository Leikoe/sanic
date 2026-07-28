# makeFunction(descriptor:completionHandler:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:completionhandler:)>

Asynchronously creates an object representing a shader function, using the specified descriptor.

## Declaration

```swift
func makeFunction(descriptor: MTLFunctionDescriptor, completionHandler: @escaping @Sendable ((any MTLFunction)?, (any Error)?) -> Void)
```

```swift
func makeFunction(descriptor: MTLFunctionDescriptor) async throws -> any MTLFunction
```

## Parameters

- **descriptor** — The description of the function object to create.
- **completionHandler** — A Swift closure or an Objective-C block that Metal calls after it creates the function.

## See also

### Creating shader function instances
- [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) — Creates an instance that represents a shader function in the library.
- [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) — Asynchronously creates a specialized shader function.
- [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)) — Synchronously creates a specialized shader function.
- [makeFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:)) — Synchronously creates an object representing a shader function, using the specified descriptor.
