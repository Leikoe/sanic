# makeBinaryFunction(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4archive/makebinaryfunction(descriptor:)>

Synchronously creates a binary version of a GPU visible function or GPU intersection function.

## Declaration

```swift
func makeBinaryFunction(descriptor: MTL4BinaryFunctionDescriptor) throws -> any MTL4BinaryFunction
```

## Parameters

- **descriptor** — A configuration that tells the method which GPU function to make into a binary function and which options to apply when compiling it.

## Return Value

A new GPU binary function instance if the method succeeds; otherwise `nil`.
