# MTLVertexBufferLayoutDescriptor

*Class · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor>

An object that configures how a render pipeline fetches data to send to the vertex function.

## Declaration

```swift
class MTLVertexBufferLayoutDescriptor
```

## Topics

### Organizing the vertex buffer layout
- [stepFunction](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stepfunction) — The circumstances under which the vertex and its attributes are presented to the vertex function.
- [stepRate](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/steprate) — The interval at which the vertex and its attributes are presented to the vertex function.
- [stride](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor/stride) — The number of bytes between the first byte of two consecutive vertices in a buffer.
- [MTLVertexStepFunction](https://developer.apple.com/documentation/metal/mtlvertexstepfunction) — The frequency with which the vertex function or post-tessellation vertex function fetches attribute data.

## See also

### Render pass inputs
- [MTLVertexDescriptor](https://developer.apple.com/documentation/metal/mtlvertexdescriptor) — An instance that describes how to organize and map data to a vertex function.
- [MTLVertexAttributeDescriptor](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor) — An object that determines how to store attribute data in memory and map it to the arguments of a vertex function.
- [MTLVertexAttributeDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray) — An array of vertex attribute descriptor instances.
- [MTLVertexBufferLayoutDescriptorArray](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray) — An array of vertex buffer layout descriptor instances.
- [MTLBufferLayoutStrideDynamic](https://developer.apple.com/documentation/metal/mtlbufferlayoutstridedynamic)
