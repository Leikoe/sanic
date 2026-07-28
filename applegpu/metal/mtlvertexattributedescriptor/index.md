# MTLVertexAttributeDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor>

An object that determines how to store attribute data in memory and map it to the arguments of a vertex function.

## Declaration

```swift
class MTLVertexAttributeDescriptor
```

## Overview

A vertex attribute descriptor provides organization information so a vertex shader function can locate and load data into its arguments. The descriptor maps memory locations to attribute locations. It supports access to multiple attributes (such as vertex coordinates, surface normals, and texture coordinates) that are interleaved within the same buffer.

## Topics

### Organizing the vertex attribute
- [format](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/format) — The format of the vertex attribute.
- [offset](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/offset) — The location of an attribute in vertex data, determined by the byte offset from the start of the vertex data.
- [bufferIndex](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor/bufferindex) — The index in the argument table for the associated vertex buffer.
- [MTLVertexFormat](https://developer.apple.com/documentation/metal/mtlvertexformat) — The vertex data format options for render pipelines.

## See also

### Render pass inputs
- [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) — An instance that describes how to organize and map data to a vertex function.
- [MTLVertexAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray) — An array of vertex attribute descriptor instances.
- [MTLVertexBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor) — An object that configures how a render pipeline fetches data to send to the vertex function.
- [MTLVertexBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray) — An array of vertex buffer layout descriptor instances.
- [MTLBufferLayoutStrideDynamic](https://developer.apple.com/documentation/metal/mtlbufferlayoutstridedynamic)
