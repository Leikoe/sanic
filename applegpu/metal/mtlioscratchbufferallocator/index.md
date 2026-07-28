# MTLIOScratchBufferAllocator

*Protocol · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator>

A protocol your app implements to provide scratch memory to an input/output command queue.

## Declaration

```swift
protocol MTLIOScratchBufferAllocator : NSObjectProtocol
```

## Overview

An allocator returns instances of [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer), another type your app implements.

## Topics

### Providing scratch memory to a queue
- [makeScratchBuffer(minimumSize:)](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator/makescratchbuffer(minimumsize:)) — Creates a scratch memory buffer for an input/output command queue.

## See also

### I/O command queues
- [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) — A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
- [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) — A configuration template you use to create a new input/output command queue.
- [MTLIOPriority](https://developer.apple.com/documentation/metal/mtliopriority) — Designates the priority for a new input/output command queue.
- [MTLIOCommandQueueType](https://developer.apple.com/documentation/metal/mtliocommandqueuetype) — Designates the queue type for a new input/output command queue.
- [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) — A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.
