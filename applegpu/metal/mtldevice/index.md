# MTLDevice

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice>

The main Metal interface to a GPU that apps use to draw graphics and run computations in parallel.

## Declaration

```swift
protocol MTLDevice : NSObjectProtocol, Sendable
```

## Overview

You can get the default [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) at runtime by calling [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()) (see [Getting the default GPU](https://developer.apple.com/documentation/metal/getting-the-default-gpu)). Each Metal device instance represents a GPU and is the main starting point for your app’s interaction with it. With a Metal device instance, you can inspect a GPU’s features and capabilities (see [Device inspection](https://developer.apple.com/documentation/metal/device-inspection)) and create subsidiary type instances with its factory methods.

- Buffers, textures, and other resources store, synchronize, and pass data between the GPU and CPU (see [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals)).

- Input/Output command queues efficiently load resources from the file system (see [Resource loading](https://developer.apple.com/documentation/metal/resource-loading)).

- Command queues create command encoders and schedule work for the GPU, including rendering and compute commands (see [Render passes](https://developer.apple.com/documentation/metal/render-passes) and [Compute passes](https://developer.apple.com/documentation/metal/compute-passes)).

- Pipeline states store render or compute pipeline configurations — which can be expensive to create — so that you can reuse them, potentially many times.

If your app uses more than one GPU (see [Multi-GPU systems](https://developer.apple.com/documentation/metal/multi-gpu-systems)), ensure that instances of these types only interact with others from the same device. For example, your app can pass a texture to a command encoder that comes from the same Metal device, but not to another device.

## Topics

### Working with GPU devices
- [Device inspection](https://developer.apple.com/documentation/metal/device-inspection) — Locate and identify a GPU and the features it supports, and sample its counters.
- [Work submission](https://developer.apple.com/documentation/metal/work-submission) — Create queues that submit work to the GPU or load assets into GPU resources, and indirect command buffers that group your frequent commands together.
- [Pipeline state creation](https://developer.apple.com/documentation/metal/pipeline-state-creation) — Create pipeline states for render and compute passes, samplers, depth and stencil states, and indirect command buffers.
- [Resource creation](https://developer.apple.com/documentation/metal/resource-creation) — Load assets with input/output queues and make various resource instances, such as buffers, textures, acceleration structures, and memory heaps.
- [Shader library and archive creation](https://developer.apple.com/documentation/metal/shader-library-and-archive-creation) — Create static and dynamic shader libraries, and binary shader archives.

### Instance Properties
- [maximumConcurrentCompilationTaskCount](https://developer.apple.com/documentation/metal/mtldevice/maximumconcurrentcompilationtaskcount) — The maximum number of concurrent compilation tasks the device is running.
- [shouldMaximizeConcurrentCompilation](https://developer.apple.com/documentation/metal/mtldevice/shouldmaximizeconcurrentcompilation) — A Boolean value that indicates whether the device uses additional CPU threads for compilation tasks.
- [supportsPlacementSparse](https://developer.apple.com/documentation/metal/mtldevice/supportsplacementsparse) — A Boolean value that indicates whether the device supports placement sparse resources.

### Instance Methods
- [functionHandle(function:)](https://developer.apple.com/documentation/metal/mtldevice/functionhandle(function:)-4bw39)
- [functionHandle(function:)](https://developer.apple.com/documentation/metal/mtldevice/functionhandle(function:)-w9ia) — Get the function handle for the specified binary-linked function from the pipeline state.
- [makeArchive(url:)](https://developer.apple.com/documentation/metal/mtldevice/makearchive(url:)) — Creates a new archive from data available at an `NSURL` address.
- [makeArgumentTable(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeargumenttable(descriptor:)) — Creates a new argument table from an argument table descriptor.
- [makeBuffer(length:options:placementSparsePageSize:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:placementsparsepagesize:)) — Creates a new placement sparse buffer of a specific length.
- [makeCommandAllocator()](https://developer.apple.com/documentation/metal/mtldevice/makecommandallocator()) — Creates a new command allocator.
- [makeCommandAllocator(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandallocator(descriptor:)) — Creates a new command allocator from a command allocator descriptor.
- [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtldevice/makecommandbuffer()) — Creates a new command buffer.
- [makeCommandQueue(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecommandqueue(descriptor:)) — Creates a command queue with the provided configuration.
- [makeCompiler(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecompiler(descriptor:)) — Creates a new compiler from a compiler descriptor.
- [makeCounterHeap(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecounterheap(descriptor:)) — Creates a new counter heap configured from a counter heap descriptor.
- [makeLogState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelogstate(descriptor:)) — Creates a shader log state with the provided configuration.
- [makeMTL4CommandQueue()](https://developer.apple.com/documentation/metal/mtldevice/makemtl4commandqueue()) — Creates a new command queue.
- [makeMTL4CommandQueue(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makemtl4commandqueue(descriptor:)) — Creates a new command queue from a queue descriptor.
- [makePipelineDataSetSerializer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makepipelinedatasetserializer(descriptor:)) — Creates a new pipeline data set serializer instance from a descriptor.
- [makeTensor(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketensor(descriptor:)) — Creates a tensor with the specified descriptor.
- [makeTensor(descriptor:attachments:)](https://developer.apple.com/documentation/metal/mtldevice/maketensor(descriptor:attachments:)) — Creates a tensor with the specified descriptor and per-plane buffer backing storage.
- [makeTextureViewPool(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/maketextureviewpool(descriptor:)) — Creates a new texture view pool from a resource view pool descriptor.
- [queryTimestampFrequency()](https://developer.apple.com/documentation/metal/mtldevice/querytimestampfrequency()) — Queries the frequency of the GPU timestamp in ticks per second.
- [size(ofCounterHeapEntry:)](https://developer.apple.com/documentation/metal/mtldevice/size(ofcounterheapentry:)) — Returns the size, in bytes, of each entry in a counter heap of a specific counter heap type when your app resolves it into a usable format.
- [tensorSizeAndAlign(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/tensorsizeandalign(descriptor:)) — Determines the size and alignment required to hold the data of a tensor you create with a descriptor in a buffer.

## See also

### Locating and inspecting a GPU device
- [Getting the default GPU](https://developer.apple.com/documentation/metal/getting-the-default-gpu) — Select the system’s default GPU device on which to run your Metal code.
- [Detecting GPU features and Metal software versions](https://developer.apple.com/documentation/metal/detecting-gpu-features-and-metal-software-versions) — Use the device object’s properties to determine how you perform tasks in Metal.
- [MTLCreateSystemDefaultDevice()](https://developer.apple.com/documentation/metal/mtlcreatesystemdefaultdevice()) — Returns the device instance Metal selects as the default.
- [Multi-GPU systems](https://developer.apple.com/documentation/metal/multi-gpu-systems) — Locate and work with internal and external GPUs and their displays, video memory, and performance tradeoffs.
