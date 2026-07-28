# MTLArgument

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargument>

Information about an argument of a graphics or compute function.

## Declaration

```swift
class MTLArgument
```

## Overview

An [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) instance describes a single argument to a Metal function. Your app uses the [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) properties to read details about a function argument as it was defined in the Metal Shading Language. You can determine the argument’s data type, access restrictions, and its associated resource type. For buffer, texture, and threadgroup memory arguments, additional properties can be read to determine more details about the argument.

Your app does not create an [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) instance directly. Creating an [MTLRenderPipelineState](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate) or [MTLComputePipelineState](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate) instance can generate a reflection instance ([MTLRenderPipelineReflection](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection) or [MTLComputePipelineReflection](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection)) that contains [MTLArgument](https://developer.apple.com/documentation/metal/mtlargument) instances.

## Topics

### Describing the argument
- [name](https://developer.apple.com/documentation/metal/mtlargument/name) — The name of the argument.
- [isActive](https://developer.apple.com/documentation/metal/mtlargument/isactive) — A Boolean that indicates whether the compiled function uses the argument.
- [index](https://developer.apple.com/documentation/metal/mtlargument/index) — The index in the argument table that corresponds to the function argument.
- [type](https://developer.apple.com/documentation/metal/mtlargument/type) — The argument’s resource type.
- [access](https://developer.apple.com/documentation/metal/mtlargument/access) — The argument’s read and/or write access.

### Describing a buffer argument
- [bufferAlignment](https://developer.apple.com/documentation/metal/mtlargument/bufferalignment) — The required byte alignment in memory for the buffer data.
- [bufferDataSize](https://developer.apple.com/documentation/metal/mtlargument/bufferdatasize) — The size, in bytes, of the buffer data.
- [bufferDataType](https://developer.apple.com/documentation/metal/mtlargument/bufferdatatype) — The data type of the buffer data.
- [bufferStructType](https://developer.apple.com/documentation/metal/mtlargument/bufferstructtype) — A description of the structure data of a buffer argument.
- [bufferPointerType](https://developer.apple.com/documentation/metal/mtlargument/bufferpointertype) — A description of the pointer to a buffer argument.

### Describing a texture argument
- [textureDataType](https://developer.apple.com/documentation/metal/mtlargument/texturedatatype) — The data type of a texture argument.
- [textureType](https://developer.apple.com/documentation/metal/mtlargument/texturetype) — The texture type of a texture argument.
- [isDepthTexture](https://developer.apple.com/documentation/metal/mtlargument/isdepthtexture) — A Boolean value that indicates whether the texture is a depth texture.

### Describing an array argument
- [arrayLength](https://developer.apple.com/documentation/metal/mtlargument/arraylength) — The number of elements, if the argument is an array.

### Describing a threadgroup memory argument
- [threadgroupMemoryAlignment](https://developer.apple.com/documentation/metal/mtlargument/threadgroupmemoryalignment) — The required byte alignment in memory for the threadgroup data.
- [threadgroupMemoryDataSize](https://developer.apple.com/documentation/metal/mtlargument/threadgroupmemorydatasize) — The size, in bytes, of the threadgroup data.

## See also

### Function arguments
- [MTLAttribute](https://developer.apple.com/documentation/metal/mtlattribute) — An object that describes an attribute defined in the stage-in argument for a shader.
- [MTLVertexAttribute](https://developer.apple.com/documentation/metal/mtlvertexattribute) — An instance that represents an attribute of a vertex function.
- [MTLAutoreleasedArgument](https://developer.apple.com/documentation/metal/mtlautoreleasedargument) — A convenience type alias for an autoreleased argument instance.
- [MTLArgumentType](https://developer.apple.com/documentation/metal/mtlargumenttype) — The resource type for an argument of a function.
- [MTLArgumentAccess](https://developer.apple.com/documentation/metal/mtlargumentaccess) — Function access restrictions to argument data in the shading language code.
