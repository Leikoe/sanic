# MTLVertexBufferLayoutDescriptorArray

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray>

An array of vertex buffer layout descriptor instances.

## Declaration

```swift
class MTLVertexBufferLayoutDescriptorArray
```

## Overview

An [MTLVertexBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray) holds an array of vertex buffer layout states. The methods of [MTLVertexBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray) set the vertex buffer layout state in the array or retrieve the state from the array.

## Topics

### Accessing a specified vertex buffer layout
- [subscript(_:)](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray/subscript(_:)) — Returns the state of the specified vertex buffer layout.

## See also

### Render pass inputs
- [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) — An instance that describes how to organize and map data to a vertex function.
- [MTLVertexAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor) — An object that determines how to store attribute data in memory and map it to the arguments of a vertex function.
- [MTLVertexAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray) — An array of vertex attribute descriptor instances.
- [MTLVertexBufferLayoutDescriptor](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor) — An object that configures how a render pipeline fetches data to send to the vertex function.
- [MTLBufferLayoutStrideDynamic](https://developer.apple.com/documentation/metal/mtlbufferlayoutstridedynamic)
