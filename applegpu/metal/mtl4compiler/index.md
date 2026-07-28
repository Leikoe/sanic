# MTL4Compiler

*Protocol · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4compiler>

A abstraction for a pipeline state and shader function compiler.

## Declaration

```swift
protocol MTL4Compiler : NSObjectProtocol, Sendable
```

## Topics

### Instance Properties
- [device](https://developer.apple.com/documentation/metal/mtl4compiler/device) — Returns the device that this compiler belongs to.
- [label](https://developer.apple.com/documentation/metal/mtl4compiler/label) — Returns the optional label you specify at creation time.
- [pipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4compiler/pipelinedatasetserializer) — Returns the pipeline data set serializer into which this compiler stores data for all pipelines it creates.

### Instance Methods
- [makeBinaryFunction(descriptor:compilerTaskOptions:)](https://developer.apple.com/documentation/metal/mtl4compiler/makebinaryfunction(descriptor:compilertaskoptions:)-5o46e) — Creates a new binary visible or intersection function synchronously.
- [makeBinaryFunction(descriptor:compilerTaskOptions:)](https://developer.apple.com/documentation/metal/mtl4compiler/makebinaryfunction(descriptor:compilertaskoptions:)-hkc4) — Creates a new binary visible or intersection function asynchronously.
- [makeComputePipelineState(descriptor:dynamicLinkingDescriptor:compilerTaskOptions:)](https://developer.apple.com/documentation/metal/mtl4compiler/makecomputepipelinestate(descriptor:dynamiclinkingdescriptor:compilertaskoptions:)-19x) — Creates a new compute pipeline state asynchronously.
- [makeComputePipelineState(descriptor:dynamicLinkingDescriptor:compilerTaskOptions:)](https://developer.apple.com/documentation/metal/mtl4compiler/makecomputepipelinestate(descriptor:dynamiclinkingdescriptor:compilertaskoptions:)-7dqdm) — Creates a new compute pipeline state object synchronously.
- [makeDynamicLibrary(library:)](https://developer.apple.com/documentation/metal/mtl4compiler/makedynamiclibrary(library:)) — Creates a new dynamic library from a library containing Metal IR code synchronously.
- [makeDynamicLibrary(url:)](https://developer.apple.com/documentation/metal/mtl4compiler/makedynamiclibrary(url:)) — Creates a new dynamic library from the contents of a file at an URL location synchronously.
- [makeLibrary(descriptor:)](https://developer.apple.com/documentation/metal/mtl4compiler/makelibrary(descriptor:)) — Creates a new Metal library synchronously.
- [makeMachineLearningPipelineState(descriptor:)](https://developer.apple.com/documentation/metal/mtl4compiler/makemachinelearningpipelinestate(descriptor:)-36hxx) — Creates a new machine learning pipeline state asynchronously.
- [makeMachineLearningPipelineState(descriptor:)](https://developer.apple.com/documentation/metal/mtl4compiler/makemachinelearningpipelinestate(descriptor:)-909v1) — Creates a new ML pipeline state with descriptor.
- [makeRenderPipelineState(descriptor:dynamicLinkingDescriptor:compilerTaskOptions:)](https://developer.apple.com/documentation/metal/mtl4compiler/makerenderpipelinestate(descriptor:dynamiclinkingdescriptor:compilertaskoptions:)-66wsk) — Creates a new render pipeline state asynchronously.
- [makeRenderPipelineState(descriptor:dynamicLinkingDescriptor:compilerTaskOptions:)](https://developer.apple.com/documentation/metal/mtl4compiler/makerenderpipelinestate(descriptor:dynamiclinkingdescriptor:compilertaskoptions:)-84kox) — Creates a new render pipeline state synchronously.
- [makeRenderPipelineStateBySpecialization(descriptor:pipeline:)](https://developer.apple.com/documentation/metal/mtl4compiler/makerenderpipelinestatebyspecialization(descriptor:pipeline:)-2636j) — Creates a new render pipeline state from another, previously unspecialized, pipeline state.
- [makeRenderPipelineStateBySpecialization(descriptor:pipeline:)](https://developer.apple.com/documentation/metal/mtl4compiler/makerenderpipelinestatebyspecialization(descriptor:pipeline:)-7s2wp) — Creates a new render pipeline state from another, previously unspecialized, pipeline state

## See also

### Shader compilation
- [Metal libraries](https://developer.apple.com/documentation/metal/metal-libraries) — Compile and manage Metal libraries from the command line.
- [Metal dynamic libraries](https://developer.apple.com/documentation/metal/metal-dynamic-libraries) — Create a single Metal library containing reusable code to reduce library size and avoid repeated shader compilation at runtime.
- [Metal binary archives](https://developer.apple.com/documentation/metal/metal-binary-archives) — Distribute precompiled GPU-specific binaries as part of your app to avoid runtime compilation of Metal shaders.
- [MTL4CompilerDescriptor](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor) — Groups together properties for creating a compiler context.
- [MTL4CompilerTaskOptions](https://developer.apple.com/documentation/metal/mtl4compilertaskoptions) — The configuration options that control the behavior of a compilation task for a Metal 4 compiler instance.
- [MTL4CompilerTaskStatus](https://developer.apple.com/documentation/metal/mtl4compilertaskstatus) — Represents the status of a compiler task.
- [MTL4Archive](https://developer.apple.com/documentation/metal/mtl4archive) — A read-only container that stores pipeline states from a shader compiler.
- [MTL4BinaryFunction](https://developer.apple.com/documentation/metal/mtl4binaryfunction) — Represents a binary function.
- [MTL4BinaryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4binaryfunctiondescriptor) — Base interface for other function-derived interfaces.
- [MTL4BinaryFunctionOptions](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions) — Options for configuring the creation of binary functions.
- [MTL4PipelineStageDynamicLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor) — Groups together properties to drive the dynamic linking process of a pipeline stage.
