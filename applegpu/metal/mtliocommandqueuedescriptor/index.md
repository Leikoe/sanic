# MTLIOCommandQueueDescriptor

*Class · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor>

A configuration template you use to create a new input/output command queue.

## Declaration

```swift
class MTLIOCommandQueueDescriptor
```

## Overview

Use this descriptor type to configure the settings of each input/output command queue that you create using [makeIOCommandQueue(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeiocommandqueue(descriptor:)). To create additional input/output command queues, you can reuse a descriptor instance and optionally reconfigure its properties.

> **Note:**
>  Changing a descriptor’s properties doesn’t affect command queues you’ve already created with the descriptor.

Create each input/output queue to meet your apps needs by setting the descriptor’s properties.

- Select a queue’s relative level of importance with the [priority](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/priority) property.

- Create a queue that runs multiple input/output command buffers in parallel by setting the [type](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/type) property to [MTLIOCommandQueueType.concurrent](https://developer.apple.com/documentation/metal/mtliocommandqueuetype/concurrent).

- Decide how many individual commands a queue can run simultaneously with the [maxCommandsInFlight](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandsinflight) property.

- Choose how many command buffers a queue can have waiting to run with [maxCommandBufferCount](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandbuffercount) property.

- Take control of the queue’s scratch memory allocation by implementing [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) and assign an instance of it to the [scratchBufferAllocator](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/scratchbufferallocator) property.

## Topics

### Configuring the input/output command queue
- [priority](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/priority) — Configures the priority for a new input/output command queue.
- [type](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/type) — Configures the queue type for a new input/output command queue.
- [maxCommandsInFlight](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandsinflight) — Sets the largest number of individual commands that an input/output command queue can run at a time.
- [maxCommandBufferCount](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandbuffercount) — Sets the largest number of outstanding input/output command buffers a queue can have at any point in time.

### Providing your own a scratch buffer
- [scratchBufferAllocator](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/scratchbufferallocator) — An optional memory allocator that you implement to manage the scratch memory that an input/output command queue requests.

## See also

### I/O command queues
- [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) — A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
- [MTLIOPriority](https://developer.apple.com/documentation/metal/mtliopriority) — Designates the priority for a new input/output command queue.
- [MTLIOCommandQueueType](https://developer.apple.com/documentation/metal/mtliocommandqueuetype) — Designates the queue type for a new input/output command queue.
- [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) — A protocol your app implements to provide scratch memory to an input/output command queue.
- [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) — A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.
