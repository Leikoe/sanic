# makeFunction(name:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)>

Creates an instance that represents a shader function in the library.

## Declaration

```swift
func makeFunction(name functionName: String) -> (any MTLFunction)?
```

## Parameters

- **functionName** — The name of the function.

## Return Value

An [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction), or `nil` if the named function isn’t found in the library.

## Discussion

If you call this method to retrieve a function that doesn’t use function constants, it returns an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance that you can use to build a render or compute pipeline.

If you call this method to retrieve a function that uses function constants to specialize its behavior, you can only use the returned instance to query the `functionConstants` property for the list of function constants. You can’t use to create a render or compute pipeline. To get a specialized instance that you can use to create a pipeline instance, call the [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) method or [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)) to generate a specialized function.

## See also

### Related Documentation
- [Metal Shading Language Guide](https://developer.apple.com/library/archive/documentation/Metal/Reference/MetalShadingLanguageGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014364)
- [Metal Programming Guide](https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40014221)

### Creating shader function instances
- [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) — Asynchronously creates a specialized shader function.
- [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:)) — Synchronously creates a specialized shader function.
- [makeFunction(descriptor:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:completionhandler:)) — Asynchronously creates an object representing a shader function, using the specified descriptor.
- [makeFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(descriptor:)) — Synchronously creates an object representing a shader function, using the specified descriptor.
