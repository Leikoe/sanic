# MTL4Archive

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4archive>

A read-only container that stores pipeline states from a shader compiler.

## Declaration

```swift
protocol MTL4Archive : NSObjectProtocol, Sendable
```

## Overview

The pipeline states can have intermediate representation (IR) binaries, GPU- and system-specific binaries, or a combination.

## Topics

### Identifying the archive
- [label](https://developer.apple.com/documentation/metal/mtl4archive/label) — A label that you can associate with this archive.

### Instance Methods
- [makeBinaryFunction(descriptor:)](https://developer.apple.com/documentation/metal/mtl4archive/makebinaryfunction(descriptor:)) — Synchronously creates a binary version of a GPU visible function or GPU intersection function.
- [makeComputePipelineState(descriptor:dynamicLinkingDescriptor:)](https://developer.apple.com/documentation/metal/mtl4archive/makecomputepipelinestate(descriptor:dynamiclinkingdescriptor:)) — Creates a compute pipeline state from the archive with a compute descriptor and a dynamic linking descriptor.
- [makeRenderPipelineState(descriptor:dynamicLinkingDescriptor:)](https://developer.apple.com/documentation/metal/mtl4archive/makerenderpipelinestate(descriptor:dynamiclinkingdescriptor:)) — Creates a render pipeline state from the archive with a render descriptor and a dynamic linking descriptor.

## See also

### Shader compilation
- [Metal libraries](https://developer.apple.com/documentation/metal/metal-libraries) — Compile and manage Metal libraries from the command line.
- [Metal dynamic libraries](https://developer.apple.com/documentation/metal/metal-dynamic-libraries) — Create a single Metal library containing reusable code to reduce library size and avoid repeated shader compilation at runtime.
- [Metal binary archives](https://developer.apple.com/documentation/metal/metal-binary-archives) — Distribute precompiled GPU-specific binaries as part of your app to avoid runtime compilation of Metal shaders.
- [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) — A abstraction for a pipeline state and shader function compiler.
- [MTL4CompilerDescriptor](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor) — Groups together properties for creating a compiler context.
- [MTL4CompilerTaskOptions](https://developer.apple.com/documentation/metal/mtl4compilertaskoptions) — The configuration options that control the behavior of a compilation task for a Metal 4 compiler instance.
- [MTL4CompilerTaskStatus](https://developer.apple.com/documentation/metal/mtl4compilertaskstatus) — Represents the status of a compiler task.
- [MTL4BinaryFunction](https://developer.apple.com/documentation/metal/mtl4binaryfunction) — Represents a binary function.
- [MTL4BinaryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4binaryfunctiondescriptor) — Base interface for other function-derived interfaces.
- [MTL4BinaryFunctionOptions](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions) — Options for configuring the creation of binary functions.
- [MTL4PipelineStageDynamicLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor) — Groups together properties to drive the dynamic linking process of a pipeline stage.
