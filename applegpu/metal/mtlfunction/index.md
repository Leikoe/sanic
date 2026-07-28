# MTLFunction

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunction>

A interface that represents a public shader function in a Metal library.

## Declaration

```swift
protocol MTLFunction : NSObjectProtocol, Sendable
```

## Overview

Use [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instances to specify which shaders a Metal pipeline calls when the GPU executes commands that specify that pipeline. For more information on creating pipeline states, see [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) and [MTLComputePipelineDescriptor](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor).

An [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance is a *specialized* function if the shader contains function constants, otherwise it is a *nonspecialized* function.

Don’t use standard allocation and initialization techniques to create an [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instance. Instead, use the function creation methods provided by the [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) protocol. To create a nonspecialized function, call the [makeFunction(name:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:)) method.

To create a specialized function, call one of these [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary) methods:

- [makeFunction(name:constantValues:completionHandler:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:))

- [makeFunction(name:constantValues:)](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:))

[MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) instances can use a significant amount of memory; release any strong references to them after you finish creating pipeline instances.

## Topics

### Identifying shader functions
- [device](https://developer.apple.com/documentation/metal/mtlfunction/device) — The device object that created the shader function.
- [label](https://developer.apple.com/documentation/metal/mtlfunction/label) — A string that identifies the shader function.
- [functionType](https://developer.apple.com/documentation/metal/mtlfunction/functiontype) — The shader function’s type.
- [name](https://developer.apple.com/documentation/metal/mtlfunction/name) — The function’s name.
- [MTLFunctionType](https://developer.apple.com/documentation/metal/mtlfunctiontype) — The type of a top-level Metal Shading Language (MSL) function.
- [options](https://developer.apple.com/documentation/metal/mtlfunction/options) — The options that Metal used to compile this function.
- [MTLFunctionOptions](https://developer.apple.com/documentation/metal/mtlfunctionoptions) — Options that define how Metal compiles a GPU function.

### Identifying the tessellation patch
- [patchType](https://developer.apple.com/documentation/metal/mtlfunction/patchtype) — The tessellation patch type of a post-tessellation vertex function.
- [patchControlPointCount](https://developer.apple.com/documentation/metal/mtlfunction/patchcontrolpointcount) — The number of patch control points in the post-tessellation vertex function.
- [MTLPatchType](https://developer.apple.com/documentation/metal/mtlpatchtype) — Types of tessellation patches that can be inputs of a post-tessellation vertex function.

### Retrieving function attributes
- [vertexAttributes](https://developer.apple.com/documentation/metal/mtlfunction/vertexattributes) — An array that describes the vertex input attributes to a vertex function.
- [stageInputAttributes](https://developer.apple.com/documentation/metal/mtlfunction/stageinputattributes) — An array that describes the input attributes to the function.

### Retrieving function constants
- [functionConstantsDictionary](https://developer.apple.com/documentation/metal/mtlfunction/functionconstantsdictionary) — A dictionary of function constants for a specialized function.

### Creating argument encoders
- [makeArgumentEncoder(bufferIndex:)](https://developer.apple.com/documentation/metal/mtlfunction/makeargumentencoder(bufferindex:)) — Creates an argument encoder for an argument buffer that’s one of this function’s arguments.
- [makeArgumentEncoder(bufferIndex:reflection:)](https://developer.apple.com/documentation/metal/mtlfunction/makeargumentencoder(bufferindex:reflection:)) — Creates an argument encoder and returns reflection information for an argument buffer that’s one of this function’s arguments

## See also

### Shader functions
- [MTLFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlfunctiondescriptor) — A description of a function object to create.
- [MTLFunctionHandle](https://developer.apple.com/documentation/metal/mtlfunctionhandle) — An object representing a function that you can add to a visible function table.
- [MTLVisibleFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor) — A specification of how to create a visible function table.
- [MTLVisibleFunctionTable](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable) — A table of shader functions visible to your app that you can pass into compute commands to customize the behavior of a shader.
- [MTLIntersectionFunctionDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiondescriptor) — A description of an intersection function that performs an intersection test.
- [MTLIntersectionFunctionTableDescriptor](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor) — A specification of how to create an intersection function table.
- [MTLIntersectionFunctionTable](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable) — A table of intersection functions that Metal calls to perform ray-tracing intersection tests.
