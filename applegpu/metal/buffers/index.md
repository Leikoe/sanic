# Buffers

*API Collection*

<https://developer.apple.com/documentation/metal/buffers>

Create and manage untyped data your app uses to exchange information with its shader functions.

## Overview

Each [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance represents a general purpose, typeless memory allocation that your app uses to send and retrieve data from a shader. Your app decides how to use and interpret the buffer’s underlying bytes.

You create buffers from either an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) or [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instance.

```swift
let deviceBuffer = device.makeBuffer(length: bufferSize,
                                     options: .storageModeShared)

let heapBuffer = heap.makeBuffer(length: bufferSize,
                                 options: .storageModePrivate)
```

```objective-c
id <MTLBuffer> deviceBuffer = [device newBufferWithLength: bufferSize
                                                  options: MTLResourceStorageModeShared];

id <MTLBuffer> heapBuffer = [heap newBufferWithLength:bufferSize
                                              options:MTLResourceStorageModePrivate];
```

```cpp
// Metal-CPP
MTL::Buffer* pDeviceBuffer = pDevice->newBuffer(bufferSize,
                                                MTL::ResourceStorageModeShared);

MTL::Buffer* pHeapBuffer = pHeap->newBuffer(bufferSize,
                                            MTL::ResourceStorageModePrivate);
```

Buffers inherently support the [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) protocol’s properties and methods, including [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode), which controls how the GPU handles its memory (see [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals)).

## Topics

### General purpose buffers
- [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) — A resource that stores data in a format defined by your app.

### Argument buffers
- [Improving CPU performance by using argument buffers](https://developer.apple.com/documentation/metal/improving-cpu-performance-by-using-argument-buffers) — Optimize your app’s performance by grouping your resources into argument buffers.
- [Managing groups of resources with argument buffers](https://developer.apple.com/documentation/metal/managing-groups-of-resources-with-argument-buffers) — Create argument buffers to organize related resources.
- [Tracking the resource residency of argument buffers](https://developer.apple.com/documentation/metal/tracking-the-resource-residency-of-argument-buffers) — Optimize resource performance within an argument buffer.
- [Indexing argument buffers](https://developer.apple.com/documentation/metal/indexing-argument-buffers) — Assign resource indices within an argument buffer.
- [Rendering terrain dynamically with argument buffers](https://developer.apple.com/documentation/metal/rendering-terrain-dynamically-with-argument-buffers) — Use argument buffers to render terrain in real time with a GPU-driven pipeline.
- [Encoding argument buffers on the GPU](https://developer.apple.com/documentation/metal/encoding-argument-buffers-on-the-gpu) — Use a compute pass to encode an argument buffer and access its arguments in a subsequent render pass.
- [Using argument buffers with resource heaps](https://developer.apple.com/documentation/metal/using-argument-buffers-with-resource-heaps) — Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
- [MTLArgumentDescriptor](https://developer.apple.com/documentation/metal/mtlargumentdescriptor) — A representation of an argument within an argument buffer.
- [MTLArgumentEncoder](https://developer.apple.com/documentation/metal/mtlargumentencoder) — An interface you can use to encode argument data into an argument buffer.
- [MTLAttributeStrideStatic](https://developer.apple.com/documentation/metal/mtlattributestridestatic)

### Model I/O interoperability
- [MTKMesh](https://developer.apple.com/documentation/MetalKit/MTKMesh) — A container for the vertex data of a Model I/O mesh, suitable for use in a Metal app.
- [MTKMeshBuffer](https://developer.apple.com/documentation/MetalKit/MTKMeshBuffer) — A buffer that backs the vertex data of a Model I/O mesh, suitable for use in a Metal app.
- [MTKMeshBufferAllocator](https://developer.apple.com/documentation/MetalKit/MTKMeshBufferAllocator) — An interface for allocating a MetalKit buffer that backs the vertex data of a Model I/O mesh, suitable for use in a Metal app.
- [MTKSubmesh](https://developer.apple.com/documentation/MetalKit/MTKSubmesh) — A container for the index data of a Model I/O submesh, suitable for use in a Metal app.
- [MTKModelError](https://developer.apple.com/documentation/MetalKit/MTKModelError) — Constants used to declare Model Errors.
- [MTKMetalVertexFormatFromModelIO(_:)](https://developer.apple.com/documentation/MetalKit/MTKMetalVertexFormatFromModelIO(_:)) — Returns a converted Metal vertex format.
- [MTKModelIOVertexFormatFromMetal(_:)](https://developer.apple.com/documentation/MetalKit/MTKModelIOVertexFormatFromMetal(_:)) — Returns a converted Model I/O vertex format.
- [MTKMetalVertexDescriptorFromModelIO(_:)](https://developer.apple.com/documentation/MetalKit/MTKMetalVertexDescriptorFromModelIO(_:)) — Returns a partially converted Metal vertex descriptor.
- [MTKModelIOVertexDescriptorFromMetal(_:)](https://developer.apple.com/documentation/MetalKit/MTKModelIOVertexDescriptorFromMetal(_:)) — Returns a partially converted Model I/O vertex descriptor.

## See also

### Resources
- [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals) — Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
- [Textures](https://developer.apple.com/documentation/metal/textures) — Create and manage typed data your app uses to exchange information with its shader functions.
- [Memory heaps](https://developer.apple.com/documentation/metal/memory-heaps) — Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.
- [Resource loading](https://developer.apple.com/documentation/metal/resource-loading) — Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.
- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) — Prevent multiple commands that can access the same resources simultaneously by coordinating those reads and writes with barriers, fences, or events.
