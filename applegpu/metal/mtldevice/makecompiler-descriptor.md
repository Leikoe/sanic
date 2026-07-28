# makeCompiler(descriptor:)

*Instance Method · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtldevice/makecompiler(descriptor:)>

Creates a new compiler from a compiler descriptor.

## Declaration

```swift
func makeCompiler(descriptor: MTL4CompilerDescriptor) throws -> any MTL4Compiler
```

## Parameters

- **descriptor** — A [MTL4CompilerDescriptor](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor) instance that configures the [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) instance.

## Return Value

A [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) instance, or `nil` if the function failed.
