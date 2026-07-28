# MTLIOPriority

*Enumeration · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliopriority>

Designates the priority for a new input/output command queue.

## Declaration

```swift
enum MTLIOPriority
```

## Overview

Set a new input/output command queue’s priority that you create with an [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) instance by setting its [priority](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/priority) property. Create a queue that minimizes an asset’s loading latency by setting a descriptor’s priority to [MTLIOPriority.high](https://developer.apple.com/documentation/metal/mtliopriority/high).

## Topics

### I/O command queue priorities
- [MTLIOPriority.normal](https://developer.apple.com/documentation/metal/mtliopriority/normal) — Designates the normal priority for a new input/output command queue.
- [MTLIOPriority.low](https://developer.apple.com/documentation/metal/mtliopriority/low) — Designates the low priority for a new input/output command queue.
- [MTLIOPriority.high](https://developer.apple.com/documentation/metal/mtliopriority/high) — Sets a new input/output command queue’s priority to a high priority.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtliopriority/init(rawvalue:))

## See also

### I/O command queues
- [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) — A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
- [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) — A configuration template you use to create a new input/output command queue.
- [MTLIOCommandQueueType](https://developer.apple.com/documentation/metal/mtliocommandqueuetype) — Designates the queue type for a new input/output command queue.
- [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) — A protocol your app implements to provide scratch memory to an input/output command queue.
- [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) — A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.
