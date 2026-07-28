# Metal libraries

<https://developer.apple.com/documentation/metal/metal-libraries>

Compile and manage Metal libraries from the command line.

## Overview

By default, your Metal shaders compile as a format called *Metal intermediate representation* (Metal IR), a GPU-independent bytecode. At your app’s runtime, Metal compiles this bytecode to a GPU-specific binary for the host device. If you provide your shader functions as strings, they first compile to Metal IR on device, and then go through a secondary compilation for GPU.

Metal source files you add to an app’s source compilation Build Phase compile to a Metal IR library named `default.metallib`. Load this library at runtime by calling the [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) in your app. For more complicated projects, you may want to create individual targets for Metal libraries, modify them in build scripts, or perform other optimizations.

Compilation of Metal IR completes before executing a shader function call. When your library consists of utility functions that other shaders use, use [Metal dynamic libraries](https://developer.apple.com/documentation/metal/metal-dynamic-libraries). To distribute GPU-specific binaries and avoid runtime shader compilation, use [Metal binary archives](https://developer.apple.com/documentation/metal/metal-binary-archives).

## Topics

### Working with Metal intermediate representation libraries
- [Building a shader library by precompiling source files](https://developer.apple.com/documentation/metal/building-a-shader-library-by-precompiling-source-files) — Create a shader library that you can add to an Xcode project with the Metal compiler tools in a command-line environment.
- [Minimizing the binary size of a shader library](https://developer.apple.com/documentation/metal/minimizing-the-binary-size-of-a-shader-library) — Reduce the storage footprint of your shaders, and potentially reduce their compile time, by selecting the Metal compiler’s size optimization option.
- [Generating and loading a Metal library symbol file](https://developer.apple.com/documentation/metal/generating-and-loading-a-metal-library-symbol-file) — Debug your Metal shaders from your production apps by creating companion symbol files at compile time and loading them at debug time.

## See also

### Shader compilation
- [Metal dynamic libraries](https://developer.apple.com/documentation/metal/metal-dynamic-libraries) — Create a single Metal library containing reusable code to reduce library size and avoid repeated shader compilation at runtime.
- [Metal binary archives](https://developer.apple.com/documentation/metal/metal-binary-archives) — Distribute precompiled GPU-specific binaries as part of your app to avoid runtime compilation of Metal shaders.
- [MTL4Compiler](https://developer.apple.com/documentation/metal/mtl4compiler) — A abstraction for a pipeline state and shader function compiler.
- [MTL4CompilerDescriptor](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor) — Groups together properties for creating a compiler context.
- [MTL4CompilerTaskOptions](https://developer.apple.com/documentation/metal/mtl4compilertaskoptions) — The configuration options that control the behavior of a compilation task for a Metal 4 compiler instance.
- [MTL4CompilerTaskStatus](https://developer.apple.com/documentation/metal/mtl4compilertaskstatus) — Represents the status of a compiler task.
- [MTL4Archive](https://developer.apple.com/documentation/metal/mtl4archive) — A read-only container that stores pipeline states from a shader compiler.
- [MTL4BinaryFunction](https://developer.apple.com/documentation/metal/mtl4binaryfunction) — Represents a binary function.
- [MTL4BinaryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4binaryfunctiondescriptor) — Base interface for other function-derived interfaces.
- [MTL4BinaryFunctionOptions](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions) — Options for configuring the creation of binary functions.
- [MTL4PipelineStageDynamicLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor) — Groups together properties to drive the dynamic linking process of a pipeline stage.
