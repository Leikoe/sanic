# makeFunction(name:constantValues:)

*Instance Method · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.12, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)>

Synchronously creates a specialized shader function.

## Declaration

```swift
func makeFunction(name: String, constantValues: MTLFunctionConstantValues) throws -> any MTLFunction
```

## Parameters

- **name** — The name of the specialized function.
- **constantValues** — The set of constant values for the function constants. The compiler can’t compile the function if any value is invalid for the function constants it requires.

## Return Value

A new [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance if the method completes successfully; otherwise Swift throws an error and Objective-C returns `nil`.

## Discussion

Function constant values are first looked up by their index, then by their name. The compiler ignores any values that don’t correspond to a function constant in the named function, and doesn’t generate errors or warnings.

## See also

### Creating shader function instances
- [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) — Creates an instance that represents a shader function in the library.
- [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) — Asynchronously creates a specialized shader function.
- [makeFunction(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:completionhandler:)) — Asynchronously creates an object representing a shader function, using the specified descriptor.
- [makeFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:)) — Synchronously creates an object representing a shader function, using the specified descriptor.
