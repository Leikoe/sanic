# Shader libraries

*API Collection*

<https://developer.apple.com/documentation/metal/shader-libraries>

Manage and load your app’s Metal shaders.

## Overview

A Metal library represents a collection of one or more shaders. Xcode creates a library from the shader source files in a project, a Metal intermediate representation (IR) file, or a binary archive file. You can also create IR files from Metal source code by running the Metal compiler in a command-line environment.

Apps create the default library instance by calling a Metal device’s [makeDefaultLibrary()](https://developer.apple.com/documentation/metal/mtldevice/makedefaultlibrary()) method. The default library contains all the shaders from a project’s shader source files, which Xcode compiles at build time. Apps create additional libraries by passing an IR file to an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeLibrary(URL:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(url:)) method or one of its sibling methods. The device can also create a library directly from source code by passing it as a string to the [makeLibrary(source:options:)](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:)) method. See [Shader library and archive creation](https://developer.apple.com/documentation/metal/shader-library-and-archive-creation) for more information.

You can apply a shader from a library to a pipeline state’s entry point, such as the [computeFunction](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor/computefunction) property for a compute pass. Start by retrieving an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance from a library, which is a reference to the library’s shader, by calling its [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) method or a sibling method. Then set the function instance to the appropriate property of a pipeline descriptor. For example, an app can retrieve a vertex stage’s entry point shader from a library and assign it to the [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexfunction) property of an [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor).

Dynamic libraries are a collection of other shaders, typically utility functions, that support the entry point shaders for a pipeline state. To create a dynamic library, pass an [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) instance to a device’s [makeDynamicLibrary(library:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(library:)) method, or pass a file URL to [makeDynamicLibrary(url:)](https://developer.apple.com/documentation/metal/mtldevice/makedynamiclibrary(url:)). Add a dynamic library to a pipeline state by including it in an array of a pipeline descriptor’s preloaded libraries property. For example, if a vertex shader calls a shader in a dynamic library, directly or indirectly, add that dynamic library to the [vertexPreloadedLibraries](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexpreloadedlibraries) property’s array. You can also build dynamic libraries with the Metal compiler in Terminal.

Binary archives are precompiled static libraries for specific GPU architectures that allow you to avoid the cost of runtime shader compilation. Because Metal automatically builds and caches shaders on the device running an app, use binary archives as part of your distributed app, or deliver them through content updates. See [Creating binary archives from device-built pipeline state objects](https://developer.apple.com/documentation/metal/creating-binary-archives-from-device-built-pipeline-state-objects) for more information on how to build and distribute binary archives for any device that supports Metal.

## Topics

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
- [MTL4BinaryFunctionOptions](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions) — Options for configuring the creation of binary functions.
- [MTL4PipelineStageDynamicLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinestagedynamiclinkingdescriptor) — Groups together properties to drive the dynamic linking process of a pipeline stage.

### Pipeline compilation
- [MTL4BlendState](https://developer.apple.com/documentation/metal/mtl4blendstate) — Enumeration for controlling the blend state of a pipeline state object.
- [MTL4FunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4functiondescriptor) — Base interface for describing a Metal 4 shader function.
- [MTL4IndirectCommandBufferSupportState](https://developer.apple.com/documentation/metal/mtl4indirectcommandbuffersupportstate) — Enumeration for controlling support for [MTLIndirectCommandBuffer](https://developer.apple.com/documentation/metal/mtlindirectcommandbuffer).
- [MTL4LibraryDescriptor](https://developer.apple.com/documentation/metal/mtl4librarydescriptor) — Serves as the base descriptor for creating a Metal library.
- [MTL4LibraryFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4libraryfunctiondescriptor) — Describes a shader function from a Metal library.
- [MTL4LogicalToPhysicalColorAttachmentMappingState](https://developer.apple.com/documentation/metal/mtl4logicaltophysicalcolorattachmentmappingstate) — Enumerates possible behaviors of how a pipeline maps its logical outputs to its color attachments.
- [MTL4NewBinaryFunctionCompletionHandler](https://developer.apple.com/documentation/metal/mtl4newbinaryfunctioncompletionhandler) — Provides a signature for a callback block that Metal calls when the compiler finishes a build task for a binary function.
- [MTL4NewMachineLearningPipelineStateCompletionHandler](https://developer.apple.com/documentation/metal/mtl4newmachinelearningpipelinestatecompletionhandler) — Provides a signature for a callback block that Metal calls when the compiler finishes a build task for a machine learning pipeline state.
- [MTL4ShaderReflection](https://developer.apple.com/documentation/metal/mtl4shaderreflection) — Option mask for requesting reflection information at pipeline build time.
- [MTL4SpecializedFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4specializedfunctiondescriptor) — Groups together properties to configure and create a specialized function by passing it to a factory method.
- [MTL4AlphaToCoverageState](https://developer.apple.com/documentation/metal/mtl4alphatocoveragestate) — Enumeration for controlling alpha-to-coverage state of a pipeline state object.
- [MTL4AlphaToOneState](https://developer.apple.com/documentation/metal/mtl4alphatoonestate) — Enumeration for controlling alpha-to-one state of a pipeline state object.
- [MTL4StaticLinkingDescriptor](https://developer.apple.com/documentation/metal/mtl4staticlinkingdescriptor) — Groups together properties to drive a static linking process.
- [MTL4StitchedFunctionDescriptor](https://developer.apple.com/documentation/metal/mtl4stitchedfunctiondescriptor) — Groups together properties that describe a shader function suitable for stitching.
- [MTLFunctionReflection](https://developer.apple.com/documentation/metal/mtlfunctionreflection) — Represents a reflection object containing information about a function in a Metal library.
- [MTLNewDynamicLibraryCompletionHandler](https://developer.apple.com/documentation/metal/mtlnewdynamiclibrarycompletionhandler)

### Pipeline harvesting
- [MTL4PipelineDataSetSerializer](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializer) — A fast-addition container for collecting data during pipeline state creation.
- [MTL4PipelineDataSetSerializerConfiguration](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerconfiguration) — Configuration options for pipeline dataset serializer objects.
- [MTL4PipelineDataSetSerializerDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedatasetserializerdescriptor) — Groups together properties to create a pipeline data set serializer.
- [MTL4PipelineDescriptor](https://developer.apple.com/documentation/metal/mtl4pipelinedescriptor) — Base type for descriptors you use for building pipeline state objects.
- [MTL4PipelineOptions](https://developer.apple.com/documentation/metal/mtl4pipelineoptions) — Provides options controlling how to compile a pipeline state.

### Shader library management
- [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) — A collection of Metal shader functions.
- [MTLDynamicLibrary](https://developer.apple.com/documentation/metal/mtldynamiclibrary) — A dynamically linkable representation of compiled shader code for a specific Metal device object.
- [MTLBinaryArchive](https://developer.apple.com/documentation/metal/mtlbinaryarchive) — A container for pipeline state descriptors and their associated compiled shader code.
- [MTLCompileOptions](https://developer.apple.com/documentation/metal/mtlcompileoptions) — Compilation settings for a Metal shader library.
- [MTLLibraryType](https://developer.apple.com/documentation/metal/mtllibrarytype) — A set of options for Metal library types.
- [MTLLanguageVersion](https://developer.apple.com/documentation/metal/mtllanguageversion) — Metal shading language versions.
- [MTLCompileSymbolVisibility](https://developer.apple.com/documentation/metal/mtlcompilesymbolvisibility)
- [MTLLibraryOptimizationLevel](https://developer.apple.com/documentation/metal/mtllibraryoptimizationlevel) — The optimization options for the Metal compiler.

### Shader functions
- [MTLFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor) — A description of a function object to create.
- [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) — A interface that represents a public shader function in a Metal library.
- [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) — An object representing a function that you can add to a visible function table.
- [MTLVisibleFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor) — A specification of how to create a visible function table.
- [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) — A table of shader functions visible to your app that you can pass into compute commands to customize the behavior of a shader.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) — A table of intersection functions that Metal calls to perform ray-tracing intersection tests.

### Stitched function libraries
- [Customizing shaders using function pointers and stitching](https://developer.apple.com/documentation/metal/customizing-shaders-using-function-pointers-and-stitching) — Define custom shader behavior at runtime by creating functions from existing ones and preferentially linking to others in a dynamic library.
- [MTLStitchedLibraryDescriptor](https://developer.apple.com/documentation/metal/mtlstitchedlibrarydescriptor) — A description of a new library of procedurally generated functions.
- [MTLFunctionStitchingGraph](https://developer.apple.com/documentation/metal/mtlfunctionstitchinggraph) — A description of a new stitched function.
- [MTLFunctionStitchingInputNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchinginputnode) — A call graph node that describes an input to the call graph.
- [MTLFunctionStitchingFunctionNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingfunctionnode) — A call graph node that describes a function call and its inputs.
- [MTLFunctionStitchingNode](https://developer.apple.com/documentation/metal/mtlfunctionstitchingnode) — A protocol to identify call graph nodes.
- [MTLFunctionStitchingAttributeAlwaysInline](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattributealwaysinline) — An attribute to specify that Metal needs to inline all of the function calls when generating the stitched function.
- [MTLFunctionStitchingAttribute](https://developer.apple.com/documentation/metal/mtlfunctionstitchingattribute) — A protocol to identify types that customize how the Metal compiler stitches a function together.

### Compile-time variant functions
- [MTLFunctionConstant](https://developer.apple.com/documentation/metal/mtlfunctionconstant) — A constant that specializes the behavior of a shader.
- [MTLFunctionConstantValues](https://developer.apple.com/documentation/metal/mtlfunctionconstantvalues) — A set of constant values that specialize a graphics or compute GPU function.

### Introspection data
- [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection) — Information about the arguments of a compute function.
- [MTLAutoreleasedComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedcomputepipelinereflection) — A convenience type alias for an autoreleased compute pipeline reflection object.
- [MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) — Information about the arguments of a graphics function.
- [MTLAutoreleasedRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlautoreleasedrenderpipelinereflection) — A convenience type alias for an autoreleased pipeline reflection instance.
- [MTLBindingType](https://developer.apple.com/documentation/metal/mtlbindingtype)
- [MTLBinding](https://developer.apple.com/documentation/metal/mtlbinding)
- [MTLBindingAccess](https://developer.apple.com/documentation/metal/mtlbindingaccess)
- [MTLBufferBinding](https://developer.apple.com/documentation/metal/mtlbufferbinding)
- [MTLTextureBinding](https://developer.apple.com/documentation/metal/mtltexturebinding)
- [MTLThreadgroupBinding](https://developer.apple.com/documentation/metal/mtlthreadgroupbinding)
- [MTLObjectPayloadBinding](https://developer.apple.com/documentation/metal/mtlobjectpayloadbinding)

### Function arguments
- [MTLAttribute](https://developer.apple.com/documentation/metal/mtlattribute) — An object that describes an attribute defined in the stage-in argument for a shader.
- [MTLVertexAttribute](https://developer.apple.com/documentation/metal/mtlvertexattribute) — An instance that represents an attribute of a vertex function.
- [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) — Information about an argument of a graphics or compute function.
- [MTLAutoreleasedArgument](https://developer.apple.com/documentation/metal/mtlautoreleasedargument) — A convenience type alias for an autoreleased argument instance.
- [MTLArgumentType](https://developer.apple.com/documentation/metal/mtlargumenttype) — The resource type for an argument of a function.
- [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess) — Function access restrictions to argument data in the shading language code.

### Shader types
- [MTLType](https://developer.apple.com/documentation/metal/mtltype) — A description of a data type.
- [MTLDataType](https://developer.apple.com/documentation/metal/mtldatatype) — The parameter type options for GPU functions, such as shaders and compute kernels.
- [MTLArrayType](https://developer.apple.com/documentation/metal/mtlarraytype) — A description of an array.
- [MTLStructType](https://developer.apple.com/documentation/metal/mtlstructtype) — A description of a structure.
- [MTLStructMember](https://developer.apple.com/documentation/metal/mtlstructmember) — An instance that provides information about a field in a structure.
- [MTLPointerType](https://developer.apple.com/documentation/metal/mtlpointertype) — A description of a pointer.
- [MTLTextureReferenceType](https://developer.apple.com/documentation/metal/mtltexturereferencetype) — A description of a texture.

### Shader logging
- [MTLLogStateDescriptor](https://developer.apple.com/documentation/metal/mtllogstatedescriptor) — An interface that represents a log state configuration.
- [MTLLogState](https://developer.apple.com/documentation/metal/mtllogstate) — A container for shader log messages.

### Errors
- [MTLLibraryError](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct) — Metal errors related to libraries.
- [MTLLibraryError.Code](https://developer.apple.com/documentation/metal/mtllibraryerror-swift.struct/code) — Error codes for Metal library errors.
- [MTLLibraryErrorDomain](https://developer.apple.com/documentation/metal/mtllibraryerrordomain) — The error domain for Metal libraries.

## See also

### Shader compilation and libraries
- [Using the Metal 4 compilation API](https://developer.apple.com/documentation/metal/using-the-metal-4-compilation-api) — Control when and how you compile an app’s shaders.
- [Using function specialization to build pipeline variants](https://developer.apple.com/documentation/metal/using-function-specialization-to-build-pipeline-variants) — Create pipelines for different levels of detail from a common shader source.
