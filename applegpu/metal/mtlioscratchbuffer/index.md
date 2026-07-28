# MTLIOScratchBuffer

*Protocol · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlioscratchbuffer>

A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.

## Declaration

```swift
protocol MTLIOScratchBuffer : NSObjectProtocol
```

## Overview

Your app can reintegrate an [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) instance’s underlying memory back into a memory pool by overriding your type’s [dealloc](https://developer.apple.com/documentation/ObjectiveC/NSObject-swift.class/dealloc) method. The system calls the method when an input/output command queue no longer needs a scratch buffer.

## Topics

### Wrapping a buffer
- [buffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer/buffer) — A Metal buffer that serves as scratch memory for an input/output command queue.

## See also

### I/O command queues
- [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) — A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
- [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) — A configuration template you use to create a new input/output command queue.
- [MTLIOPriority](https://developer.apple.com/documentation/metal/mtliopriority) — Designates the priority for a new input/output command queue.
- [MTLIOCommandQueueType](https://developer.apple.com/documentation/metal/mtliocommandqueuetype) — Designates the queue type for a new input/output command queue.
- [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) — A protocol your app implements to provide scratch memory to an input/output command queue.
