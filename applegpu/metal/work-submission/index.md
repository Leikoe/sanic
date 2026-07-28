# Work submission

*API Collection*

<https://developer.apple.com/documentation/metal/work-submission>

Create queues that submit work to the GPU or load assets into GPU resources, and indirect command buffers that group your frequent commands together.

## Topics

### Creating command queues
- [makeCommandQueue()](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue()) — Creates a queue you use to submit rendering and computation commands to a GPU.
- [makeCommandQueue(maxCommandBufferCount:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(maxcommandbuffercount:)) — Creates a queue you use to submit rendering and computation commands to a GPU that has a fixed number of uncompleted command buffers.

### Creating residency sets
- [makeResidencySet(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeresidencyset(descriptor:)) — Creates a residency set, which can move resources in and out of memory residency.

### Creating I/O command queues
- [makeIOCommandQueue(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeiocommandqueue(descriptor:)) — Creates an input/output command queue you use to submit commands that load assets from the file system into GPU resources or system memory.

### Creating I/O file handles
- [makeIOFileHandle(url:)](https://developer.apple.com/documentation/metal/mtldevice/makeiofilehandle(url:)) — Creates an input/output file handle instance that represents a file at a URL.
- [makeIOFileHandle(url:compressionMethod:)](https://developer.apple.com/documentation/metal/mtldevice/makeiofilehandle(url:compressionmethod:)) — Creates an input/output file handle instance that represents a compressed file at a URL.
- [makeIOHandle(url:)](https://developer.apple.com/documentation/metal/mtldevice/makeiohandle(url:)) — Creates an input/output file handle instance that represents a file at a URL.
- [makeIOHandle(url:compressionMethod:)](https://developer.apple.com/documentation/metal/mtldevice/makeiohandle(url:compressionmethod:)) — Creates an input/output file handle instance that represents a compressed file at a URL.

### Creating indirect command buffers
- [makeIndirectCommandBuffer(descriptor:maxCommandCount:options:)](https://developer.apple.com/documentation/metal/mtldevice/makeindirectcommandbuffer(descriptor:maxcommandcount:options:)) — Creates an indirect command buffer instance.

## See also

### Working with GPU devices
- [Device inspection](https://developer.apple.com/documentation/metal/device-inspection) — Locate and identify a GPU and the features it supports, and sample its counters.
- [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation) — Create pipeline states for render and compute passes, samplers, depth and stencil states, and indirect command buffers.
- [Resource creation](https://developer.apple.com/documentation/metal/resource-creation) — Load assets with input/output queues and make various resource instances, such as buffers, textures, acceleration structures, and memory heaps.
- [Shader library and archive creation](https://developer.apple.com/documentation/metal/shader-library-and-archive-creation) — Create static and dynamic shader libraries, and binary shader archives.
