# Resource loading

*API Collection*

<https://developer.apple.com/documentation/metal/resource-loading>

Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.

## Overview

Metal 3 adds input/output command queues and buffers that make the most of a device’s storage hardware, including flash storage and the unified memory architecture of Apple silicon, when available. When you run a dedicated input/output queue alongside your GPU tasks, you can synchronize them with Metal shared events. With this approach, you can minimize load screen times by fetching the essential assets first and streaming the rest as you need them. You can also start multiple input/output command buffers to load different asset batches and later cancel the ones you don’t need. Ensure that time-sensitive assets, such as sound effects, load with lower latency by running those command buffers on higher-priority queues that you create.

First, create [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) instances by configuring an [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) instance and passing it to an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeIOCommandQueue(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeiocommandqueue(descriptor:)) method.

```swift
// Create a Metal I/O command queue.
let commandQueueDescriptor = MTLIOCommandQueueDescriptor()

commandQueueDescriptor.type = .concurrent
commandQueueDescriptor.priority = .normal

let ioCommandQueue = try device.makeIOCommandQueue(descriptor:
                                                    commandQueueDescriptor)
```

```objective-c
// Create a Metal I/O command queue.
MTLIOCommandQueueDescriptor *commandQueueDescriptor;
commandQueueDescriptor = [[MTLIOCommandQueueDescriptor alloc] init];

commandQueueDescriptor.type = MTLIOCommandQueueTypeConcurrent;
commandQueueDescriptor.priority = MTLIOPriorityNormal;

NSError *error = nil;
id<MTLIOCommandQueue> ioCommandQueue;
ioCommandQueue = [device newIOCommandQueueWithDescriptor:commandQueueDescriptor
                                                   error:&error];

if (error != nil) {
        // Report the error.
        ...
}
```

For each queue, create one or more [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) instances by calling the queue’s [makeCommandBuffer()](https://developer.apple.com/documentation/metal/mtliocommandqueue/makecommandbuffer()) or [makeCommandBufferWithUnretainedReferences()](https://developer.apple.com/documentation/metal/mtliocommandqueue/makecommandbufferwithunretainedreferences()) method. For each command buffer, load the assets you want by calling any of the [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) protocol’s load methods. For example:

- The [load(_:offset:size:sourceHandle:sourceHandleOffset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/load(_:offset:size:sourcehandle:sourcehandleoffset:)) method loads an asset into an [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer).

- The [load(_:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/load(_:slice:level:size:sourcebytesperrow:sourcebytesperimage:destinationorigin:sourcehandle:sourcehandleoffset:)) method loads an asset into an [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture).

- The [loadBytes(_:size:sourceHandle:sourceHandleOffset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/loadbytes(_:size:sourcehandle:sourcehandleoffset:)) method loads an asset, such as an audio file, into a CPU-accessible memory buffer.

```swift
// Create a Metal I/O command buffer.
let ioCommandBuffer = ioCommandQueue.makeCommandBuffer()

// Encode a command that loads a texture.
ioCommandBuffer.load(texture,
                     slice: 0,
                     level: 0,
                     size: textureSize,
                     sourceBytesPerRow: bytesPerRow,
                     sourceBytesPerImage: bytesPerImage,
                     destinationOrigin: origin,
                     sourceHandle: fileHandle,
                     sourceHandleOffset: 0)

// Encode a command that loads a buffer.
ioCommandBuffer.load(buffer,
                     offset: 0,
                     size: bufferSize,
                     sourceHandle: fileHandle,
                     sourceHandleOffset: 0)

// Submit the command buffer to run.
ioCommandBuffer.commit()
```

```objective-c
// Create a Metal I/O command buffer.
id<MTLIOCommandBuffer> ioCommandBuffer = [ioCommandQueue commandBuffer];

// Encode a command that loads a texture.
[ioCommandBuffer loadTexture:texture
                       slice:0
                       level:0
                        size:textureSize
           sourceBytesPerRow:bytesPerRow
         sourceBytesPerImage:bytesPerImage
           destinationOrigin:origin
                sourceHandle:textureAssetHandle
          sourceHandleOffset:0];

// Encode a command that loads a buffer.
[ioCommandBuffer loadBuffer:buffer
                     offset:0
                       size:bufferSize
               sourceHandle:bufferAssetHandle
         sourceHandleOffset:0];


// Submit the command buffer to run.
[ioCommandBuffer commit];
```

For each asset, create an [MTLIOFileHandle](https://developer.apple.com/documentation/metal/mtliofilehandle) instance using the input/output command buffer’s load methods. To create a file handle for your asset, call an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [makeIOHandle(url:)](https://developer.apple.com/documentation/metal/mtldevice/makeiohandle(url:)) or [makeIOHandle(url:compressionMethod:)](https://developer.apple.com/documentation/metal/mtldevice/makeiohandle(url:compressionmethod:)) method.

```swift
func createHandleForFile(at url: URL, with device: MTLDevice) -> MTLIOFileHandle? {
    return try? device.makeIOHandle(url: url)
}
```

```objective-c
id<MTLIOFileHandle> createHandleForFile(NSURL *url, id<MTLDevice> device)
{
    NSError *error = nil;
    id<MTLIOFileHandle> assetHandle = [device newIOHandleWithURL:url error:&error];

    if (error != nil) {
        // Report the error.
        ...
    }

    return assetHandle;
}
```

> **Note:**
>  You need to create each file handle using the same [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance that created the [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) and [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) instances that load the files.

To help minimize your appʼs storage footprint, compress your assets at development time. First, create a new compression context with the [MTLIOCreateCompressionContext](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext) function. Then, add data for an asset to the compression context using the [MTLIOCompressionContextAppendData(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)) function. Finally, call the  [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) function to save the context to a compressed file that you add to your project.

## Topics

### I/O command queues
- [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) — A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
- [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) — A configuration template you use to create a new input/output command queue.
- [MTLIOPriority](https://developer.apple.com/documentation/metal/mtliopriority) — Designates the priority for a new input/output command queue.
- [MTLIOCommandQueueType](https://developer.apple.com/documentation/metal/mtliocommandqueuetype) — Designates the queue type for a new input/output command queue.
- [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) — A protocol your app implements to provide scratch memory to an input/output command queue.
- [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) — A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.

### I/O command buffers
- [MTLIOCommandBuffer](https://developer.apple.com/documentation/metal/mtliocommandbuffer) — A command buffer that contains input/output commands that work with files in the file systems and Metal resources.
- [MTLIOFileHandle](https://developer.apple.com/documentation/metal/mtliofilehandle) — Represents a raw or compressed file, such as a resource asset file in your app’s bundle.
- [MTLIOCommandBufferHandler](https://developer.apple.com/documentation/metal/mtliocommandbufferhandler) — A convenience type that defines the signature of an input/output command buffer’s completion handler.
- [MTLIOStatus](https://developer.apple.com/documentation/metal/mtliostatus) — Represents the state of an input/output command buffer.
- [MTLIOError.Code](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct/code) — The error codes for creating an input/output file handle.
- [MTLIOErrorDomain](https://developer.apple.com/documentation/metal/mtlioerrordomain) — The domain for input/output command queue errors.

### Asset compression
- [MTLIOCreateCompressionContext(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocreatecompressioncontext(_:_:_:)) — Creates a compression context that you use to compress data into a single file.
- [MTLIOCompressionMethod](https://developer.apple.com/documentation/metal/mtliocompressionmethod) — The compression codecs that Metal supports for input/output handles.
- [MTLIOCompressionContextDefaultChunkSize()](https://developer.apple.com/documentation/metal/mtliocompressioncontextdefaultchunksize()) — Returns a compression chunk size you can use as a default for creating a compression context.
- [MTLIOCompressionContext](https://developer.apple.com/documentation/metal/mtliocompressioncontext) — A pointer that represents the state of a file compression session in progress.
- [MTLIOCompressionContextAppendData(_:_:_:)](https://developer.apple.com/documentation/metal/mtliocompressioncontextappenddata(_:_:_:)) — Adds data to a compression context.
- [MTLIOFlushAndDestroyCompressionContext(_:)](https://developer.apple.com/documentation/metal/mtlioflushanddestroycompressioncontext(_:)) — Finishes compressing and saves the file that a compression context represents.
- [MTLIOCompressionStatus](https://developer.apple.com/documentation/metal/mtliocompressionstatus) — Represents the final state of a compression context.

## See also

### Resources
- [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals) — Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
- [Buffers](https://developer.apple.com/documentation/metal/buffers) — Create and manage untyped data your app uses to exchange information with its shader functions.
- [Textures](https://developer.apple.com/documentation/metal/textures) — Create and manage typed data your app uses to exchange information with its shader functions.
- [Memory heaps](https://developer.apple.com/documentation/metal/memory-heaps) — Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.
- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) — Prevent multiple commands that can access the same resources simultaneously by coordinating those reads and writes with barriers, fences, or events.
