# Metal dynamic libraries

<https://developer.apple.com/documentation/metal/metal-dynamic-libraries>

Create a single Metal library containing reusable code to reduce library size and avoid repeated shader compilation at runtime.

## Overview

As shaders grow in size, complexity, and scope, they often end up sharing utility functions. Under the default compilation model in Metal, linking embeds libraries, similar to static linking with the LLVM linker. For Metal, embedding libraries in this manner has two consequences: an increase in binary size, and an increase in compilation time. As each library loads, it compiles its own version of any utility functions, meaning Metal compiles and duplicates your utility functions multiple times.

To avoid this problem, Metal offers dynamic libraries, similar to an LLVM dynamically shared library. Your app loads and compiles dynamic libraries for the device GPU once, the first time a shader requests them. Subsequent shader calls use these compiled utility functions instead of compiling a separate version of the same shader binary.

To support Metal dynamic libraries in your app, call [makeDynamicLibrary(library:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(library:)) with a dynamic library that you bundle as part of your app. Then add it to a pipeline descriptor’s dynamic library information through a property like [preloadedLibraries](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/preloadedlibraries).

## Topics

### Working with Metal dynamic libraries
- [Compiling and linking Metal dynamic libraries](https://developer.apple.com/documentation/metal/compiling-and-linking-metal-dynamic-libraries) — Build a Metal dynamic library from the command line, allowing for runtime loading of shared shaders.
- [Creating a Metal dynamic library](https://developer.apple.com/documentation/metal/creating-a-metal-dynamic-library) — Compile a library of shaders and write it to a file as a dynamically linked library.

## See also

### Shader compilation
- [Metal libraries](https://developer.apple.com/documentation/metal/metal-libraries) — Compile and manage Metal libraries from the command line.
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
