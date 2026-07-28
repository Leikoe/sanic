# MTLArgumentDescriptor

*Class · iOS 11.0, iPadOS 11.0, Mac Catalyst 13.1, macOS 10.13, tvOS 11.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlargumentdescriptor>

A representation of an argument within an argument buffer.

## Declaration

```swift
class MTLArgumentDescriptor
```

## Overview

This descriptor can represent arguments within flat structures only. It can represent arrays of allowed argument buffer data types, but it cannot represent arguments within nested structures. Argument buffers with simple, flat structures can be represented by an array of [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) instances. You can then use this array to create an [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) instance by calling the [makeArgumentEncoder(arguments:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumentencoder(arguments:)) method. Argument buffers with complex, nested structures need to define their structure in Metal shading language code, which can then be directly assigned to a specific buffer index of a function. You can then use this buffer index to call the [makeArgumentEncoder(bufferIndex:)](https://developer.apple.com/documentation/metal/mtlfunction/makeargumentencoder(bufferindex:)) method and create an [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) instance.

## Topics

### Setting the descriptor’s properties
- [dataType](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/datatype) — The data type of the argument.
- [index](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/index) — The index ID of the argument.
- [access](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/access) — The access permissions of the argument.
- [arrayLength](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/arraylength) — The length of an array argument.
- [constantBlockAlignment](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/constantblockalignment) — The alignment of the constant block.
- [textureType](https://developer.apple.com/documentation/metal/mtlargumentdescriptor/texturetype) — The texture type of a texture argument.

## See also

### Argument buffers
- [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) — Optimize your app’s performance by grouping your resources into argument buffers.
- [Managing groups of resources with argument buffers](https://developer.apple.com/documentation/metal/managing-groups-of-resources-with-argument-buffers) — Create argument buffers to organize related resources.
- [Tracking the resource residency of argument buffers](https://developer.apple.com/documentation/metal/tracking-the-resource-residency-of-argument-buffers) — Optimize resource performance within an argument buffer.
- [Indexing argument buffers](https://developer.apple.com/documentation/metal/indexing-argument-buffers) — Assign resource indices within an argument buffer.
- [Rendering terrain dynamically with argument buffers](https://developer.apple.com/documentation/metal/rendering-terrain-dynamically-with-argument-buffers) — Use argument buffers to render terrain in real time with a GPU-driven pipeline.
- [Encoding argument buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-argument-buffers-on-the-gpu) — Use a compute pass to encode an argument buffer and access its arguments in a subsequent render pass.
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) — An interface you can use to encode argument data into an argument buffer.
- [MTLAttributeStrideStatic](https://developer.apple.com/documentation/metal/mtlattributestridestatic)
