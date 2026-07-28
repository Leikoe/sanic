# MTLBuffer

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlbuffer>

A resource that stores data in a format defined by your app.

## Declaration

```swift
protocol MTLBuffer : MTLResource
```

## Overview

An [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer) instance can be used only with the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that created it. Don’t implement this protocol yourself; instead, use the following [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) methods to create `MTLBuffer` instances:

- [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:)) creates a `MTLBuffer` instance with a new storage allocation.

- [makeBuffer(bytes:length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytes:length:options:)) creates a `MTLBuffer` instance by copying data from an existing storage allocation into a new allocation.

- [makeBuffer(bytesNoCopy:length:options:deallocator:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(bytesnocopy:length:options:deallocator:)) creates a `MTLBuffer` instance that reuses an existing storage allocation and does not allocate any new storage.

The Metal framework doesn’t know anything about the contents of an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), just its size. You define the format of the data in the buffer and ensure that your app and your shaders know how to read and write the data. For example, you might create a struct in your shader that defines the data you want to store in the buffer and its memory layout.

If you create a buffer with a managed resource storage mode ([MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed)), you need to call [didModifyRange:](https://developer.apple.com/documentation/metal/mtlbuffer/didmodifyrange:) to tell Metal to copy any changes to the GPU.

## Topics

### Creating a texture that shares buffer data
- [makeTexture(descriptor:offset:bytesPerRow:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketexture(descriptor:offset:bytesperrow:)) — Creates a texture that shares its storage with the buffer.

### Reading the buffer’s data on the CPU
- [contents()](https://developer.apple.com/documentation/metal/mtlbuffer/contents()) — Gets the system address of the buffer’s storage allocation.

### Synchronizing data to the GPU for managed buffers
- [didModifyRange(_:)](https://developer.apple.com/documentation/metal/mtlbuffer/didmodifyrange(_:)) — Informs the GPU that the CPU has modified a section of the buffer.

### Debugging buffers
- [addDebugMarker(_:range:)](https://developer.apple.com/documentation/metal/mtlbuffer/adddebugmarker(_:range:)) — Adds a debug marker string to a specific buffer range.
- [removeAllDebugMarkers()](https://developer.apple.com/documentation/metal/mtlbuffer/removealldebugmarkers()) — Removes all debug marker strings from the buffer.

### Reading buffer length
- [length](https://developer.apple.com/documentation/metal/mtlbuffer/length) — The logical size of the buffer, in bytes.

### Creating views of buffers on other GPUs
- [makeRemoteBufferView(_:)](https://developer.apple.com/documentation/metal/mtlbuffer/makeremotebufferview(_:)) — Creates a remote view of the buffer for another GPU in the same peer group.
- [remoteStorageBuffer](https://developer.apple.com/documentation/metal/mtlbuffer/remotestoragebuffer) — The buffer on another GPU that the buffer was created from, if any.

### Instance Properties
- [gpuAddress](https://developer.apple.com/documentation/metal/mtlbuffer/gpuaddress)
- [sparseBufferTier](https://developer.apple.com/documentation/metal/mtlbuffer/sparsebuffertier)

### Instance Methods
- [makeTensor(descriptor:offset:)](https://developer.apple.com/documentation/metal/mtlbuffer/maketensor(descriptor:offset:)) — Creates a single-plane tensor with the specified descriptor that shares storage with this buffer.
