# MTLVertexDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexdescriptor>

An instance that describes how to organize and map data to a vertex function.

## Declaration

```swift
class MTLVertexDescriptor
```

## Overview

An [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) instance is used to configure how vertex data stored in memory is mapped to attributes in a vertex shader.

A pipeline state is the state of the graphics rendering pipeline, including shaders, blending, multisampling, and visibility testing. For every pipeline state, there can be only one [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) instance. When you configure an [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) instance to create this pipeline state, you use an [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) instance to establish the vertex layout for the function associated with the pipeline. Create and configure an [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) instance, then use this instance to set the [vertexDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/vertexdescriptor) property of the [MTLRenderPipelineDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor) instance.

## Topics

### Setting default values
- [reset()](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/reset()) — Resets the default state for the vertex descriptor.

### Accessing the vertex buffer layouts and vertex attributes
- [attributes](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/attributes) — An array of state data that describes how vertex attribute data is stored in memory and is mapped to arguments for a vertex shader function.
- [layouts](https://developer.apple.com/documentation/metal/mtlvertexdescriptor/layouts) — An array of state data that describes how data are fetched by a vertex shader function when rendering primitives.

## See also

### Render pass inputs
- [MTLVertexAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor) — An object that determines how to store attribute data in memory and map it to the arguments of a vertex function.
- [MTLVertexAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray) — An array of vertex attribute descriptor instances.
- [MTLVertexBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor) — An object that configures how a render pipeline fetches data to send to the vertex function.
- [MTLVertexBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray) — An array of vertex buffer layout descriptor instances.
- [MTLBufferLayoutStrideDynamic](https://developer.apple.com/documentation/metal/mtlbufferlayoutstridedynamic)
