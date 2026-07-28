# MTL4BinaryFunction

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4binaryfunction>

Represents a binary function.

## Declaration

```swift
protocol MTL4BinaryFunction : NSObjectProtocol, Sendable
```

## Overview

A binary function is a shader that you precompile from Metal IR to GPU machine code.

## Topics

### Instance Properties
- [functionType](https://developer.apple.com/documentation/metal/mtl4binaryfunction/functiontype) — Describes the type of this binary function.
- [name](https://developer.apple.com/documentation/metal/mtl4binaryfunction/name) — Obtains the optional name of this binary function.

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
- [MTL4BinaryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4binaryfunctiondescriptor) — Base interface for other function-derived interfaces.
- [MTL4BinaryFunctionOptions](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions) — Options for configuring the creation of binary functions.
- [MTL4PipelineStageDynamicLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor) — Groups together properties to drive the dynamic linking process of a pipeline stage.
