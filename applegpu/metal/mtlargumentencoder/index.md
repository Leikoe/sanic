# MTLArgumentEncoder

*Protocol · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentencoder>

An interface you can use to encode argument data into an argument buffer.

## Declaration

```swift
protocol MTLArgumentEncoder : NSObjectProtocol
```

## Overview

An [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) instance encodes buffers, textures, samplers, and inlined constant data into an argument buffer. An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance represents the argument buffer that you set as the encoding destination by calling the [setArgumentBuffer(_:offset:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:offset:)) method.

The recommended way to declare an argument buffer is to define its structure in your Metal shading language code. You can assign the argument buffer to a function’s specific buffer index. To create an encoder for this type of argument buffer, call one of the following [MTLFunction](https://developer.apple.com/documentation/metal/mtlfunction) methods:

- [makeArgumentEncoder(bufferIndex:)](https://developer.apple.com/documentation/metal/mtlfunction/makeargumentencoder(bufferindex:))

- [makeArgumentEncoder(bufferIndex:reflection:)](https://developer.apple.com/documentation/metal/mtlfunction/makeargumentencoder(bufferindex:reflection:))

If you construct your shaders dynamically at runtime, you can still construct argument buffers as parameters for the shader. Define each argument separately and then add it to an array of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances. To create an encoder for this type of argument buffer, call the [makeArgumentEncoder(arguments:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)) method of the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) class.

> **Important:**
>  A runtime validation error occurs if you create a `MTLArgumentEncoder` instance using structures that don’t reference any other resources and don’t provide any `[[id()]]` annotation on any of their members.

## Topics

### Creating an argument buffer
- [setArgumentBuffer(_:offset:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:offset:)) — Specifies the position in a buffer where the encoder writes argument data.
- [setArgumentBuffer(_:startOffset:arrayElement:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setargumentbuffer(_:startoffset:arrayelement:)) — Specifies an array element within a buffer where the encoder writes argument data.
- [encodedLength](https://developer.apple.com/documentation/metal/mtlargumentencoder/encodedlength) — The number of bytes required to store the encoded resources of an argument buffer.

### Encoding buffers
- [setBuffer(_:offset:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setbuffer(_:offset:index:)) — Encodes a reference to a buffer into the argument buffer.
- [setBuffers(_:offsets:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setbuffers(_:offsets:range:)) — Encodes references to an array of buffers into the argument buffer.

### Encoding textures
- [setTexture(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/settexture(_:index:)) — Encodes a reference to a texture into the argument buffer.
- [setTextures(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/settextures(_:range:)) — Encodes references to an array of textures into the argument buffer.

### Encoding samplers
- [setSamplerState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setsamplerstate(_:index:)) — Encodes a sampler into the argument buffer.
- [setSamplerStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setsamplerstates(_:range:)) — Encodes an array of samplers into the argument buffer.

### Encoding pipeline states
- [setRenderPipelineState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setrenderpipelinestate(_:index:)) — Encodes a reference to a render pipeline state into the argument buffer.
- [setRenderPipelineStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setrenderpipelinestates(_:range:)) — Encodes references to an array of render pipeline states into the argument buffer.
- [setComputePipelineState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestate(_:index:)) — Encodes a reference to a compute pipeline state into the argument buffer.
- [setComputePipelineStates(_:with:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestates(_:with:)) — Encodes references to an array of compute pipeline states into the argument buffer.
- [setComputePipelineState(_:at:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestate(_:at:)) — Encodes a reference to a compute pipeline state into the argument buffer.
- [setComputePipelineStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setcomputepipelinestates(_:range:)) — Encodes references to an array of compute pipeline states into the argument buffer.

### Encoding inlined constant data
- [constantData(at:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/constantdata(at:)) — Returns a pointer to an inline, constant-data argument within the argument buffer.

### Encoding indirect command buffers
- [setIndirectCommandBuffer(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setindirectcommandbuffer(_:index:)) — Encodes a reference to an indirect command buffer into the argument buffer.
- [setIndirectCommandBuffers(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setindirectcommandbuffers(_:range:)) — Encodes an array of indirect command buffers into the argument buffer.

### Encoding acceleration structures
- [setAccelerationStructure(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setaccelerationstructure(_:index:)) — Encodes a reference to an acceleration structure into the argument buffer.

### Encoding function tables
- [setVisibleFunctionTable(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setvisiblefunctiontable(_:index:)) — Encodes a reference to a visible-function table into the argument buffer.
- [setIntersectionFunctionTable(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setintersectionfunctiontable(_:index:)) — Encodes a reference to a ray-tracing intersection-function table into the argument buffer.
- [setIntersectionFunctionTables(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setintersectionfunctiontables(_:range:)) — Encodes references to an array of ray-tracing intersection-function tables into the argument buffer.
- [setVisibleFunctionTables(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setvisiblefunctiontables(_:range:)) — Encodes references to an array of ray-tracing intersection-function tables into the argument buffer.

### Creating a nested argument encoder
- [makeArgumentEncoderForBuffer(atIndex:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/makeargumentencoderforbuffer(atindex:)) — Creates a new argument encoder for a nested argument buffer.

### Querying alignment
- [alignment](https://developer.apple.com/documentation/metal/mtlargumentencoder/alignment) — The alignment, in bytes, required for storing the encoded resources of an argument buffer.

### Identifying the argument encoder
- [label](https://developer.apple.com/documentation/metal/mtlargumentencoder/label) — A string that identifies the argument buffer.
- [device](https://developer.apple.com/documentation/metal/mtlargumentencoder/device) — The device object that created the argument encoder.

### Instance Methods
- [setDepthStencilState(_:index:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setdepthstencilstate(_:index:))
- [setDepthStencilStates(_:range:)](https://developer.apple.com/documentation/metal/mtlargumentencoder/setdepthstencilstates(_:range:))

## See also

### Argument buffers
- [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) — Optimize your app’s performance by grouping your resources into argument buffers.
- [Managing groups of resources with argument buffers](https://developer.apple.com/documentation/metal/managing-groups-of-resources-with-argument-buffers) — Create argument buffers to organize related resources.
- [Tracking the resource residency of argument buffers](https://developer.apple.com/documentation/metal/tracking-the-resource-residency-of-argument-buffers) — Optimize resource performance within an argument buffer.
- [Indexing argument buffers](https://developer.apple.com/documentation/metal/indexing-argument-buffers) — Assign resource indices within an argument buffer.
- [Rendering terrain dynamically with argument buffers](https://developer.apple.com/documentation/metal/rendering-terrain-dynamically-with-argument-buffers) — Use argument buffers to render terrain in real time with a GPU-driven pipeline.
- [Encoding argument buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-argument-buffers-on-the-gpu) — Use a compute pass to encode an argument buffer and access its arguments in a subsequent render pass.
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) — A representation of an argument within an argument buffer.
- [MTLAttributeStrideStatic](https://developer.apple.com/documentation/metal/mtlattributestridestatic)
