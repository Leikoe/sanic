# makeBinaryFunction(descriptor:compilerTaskOptions:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4compiler/makebinaryfunction(descriptor:compilertaskoptions:)-5o46e>

Creates a new binary visible or intersection function synchronously.

## Declaration

```swift
func makeBinaryFunction(descriptor: MTL4BinaryFunctionDescriptor, compilerTaskOptions: MTL4CompilerTaskOptions? = nil) throws -> any MTL4BinaryFunction
```

## Parameters

- **descriptor** — A binary function descriptor to use for creating the binary function.
- **compilerTaskOptions** — A descriptor of the compilation itself, providing parameters that influence execution of the compilation process.

## Return Value

A binary function upon success, otherwise this function throws.
