# MTL4BinaryFunctionOptions

*Structure · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions>

Options for configuring the creation of binary functions.

## Declaration

```swift
struct MTL4BinaryFunctionOptions
```

## Topics

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions/init(rawvalue:))

### Type Properties
- [pipelineIndependent](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions/pipelineindependent) — Compiles the function to have its function handles return a constant MTLResourceID across all pipeline states. The function needs to be linked to the pipeline that will use this function.

## See also

### Shader compilation
- [Metal libraries](https://developer.apple.com/documentation/metal/metal-libraries) — Compile and manage Metal libraries from the command line.
- [Metal dynamic libraries](https://developer.apple.com/documentation/metal/metal-dynamic-libraries) — Create a single Metal library containing reusable code to reduce library size and avoid repeated shader compilation at runtime.
- [Metal binary archives](https://developer.apple.com/documentation/metal/metal-binary-archives) — Distribute precompiled GPU-specific binaries as part of your app to avoid runtime compilation of Metal shaders.
- [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) — A abstraction for a pipeline state and shader function compiler.
- [MTL4CompilerDescriptor](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor) — Groups together properties for creating a compiler context.
- [MTL4CompilerTaskOptions](https://developer.apple.com/documentation/metal/mtl4compilertaskoptions) — The configuration options that control the behavior of a compilation task for a Metal 4 compiler instance.
- [MTL4CompilerTaskStatus](https://developer.apple.com/documentation/metal/mtl4compilertaskstatus) — Represents the status of a compiler task.
- [MTL4Archive](https://developer.apple.com/documentation/metal/mtl4archive) — A read-only container that stores pipeline states from a shader compiler.
- [MTL4BinaryFunction](https://developer.apple.com/documentation/metal/mtl4binaryfunction) — Represents a binary function.
- [MTL4BinaryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4binaryfunctiondescriptor) — Base interface for other function-derived interfaces.
- [MTL4PipelineStageDynamicLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor) — Groups together properties to drive the dynamic linking process of a pipeline stage.
