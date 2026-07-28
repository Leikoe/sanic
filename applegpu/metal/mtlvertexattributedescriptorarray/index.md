# MTLVertexAttributeDescriptorArray

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray>

An array of vertex attribute descriptor instances.

## Declaration

```swift
class MTLVertexAttributeDescriptorArray
```

## Overview

An [MTLVertexAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray) instance is an array of instances that defines how vertex attribute data is formatted and assigned to an index in the attribute argument table. The methods of [MTLVertexAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray) set or retrieve the attribute formatting information from the array.

## Topics

### Accessing a specified vertex attribute
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray/subscript(_:)) — Returns the state of the specified vertex attribute.

## See also

### Render pass inputs
- [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) — An instance that describes how to organize and map data to a vertex function.
- [MTLVertexAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor) — An object that determines how to store attribute data in memory and map it to the arguments of a vertex function.
- [MTLVertexBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor) — An object that configures how a render pipeline fetches data to send to the vertex function.
- [MTLVertexBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray) — An array of vertex buffer layout descriptor instances.
- [MTLBufferLayoutStrideDynamic](https://developer.apple.com/documentation/metal/mtlbufferlayoutstridedynamic)
