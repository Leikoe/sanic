# MTLIOCommandQueueType

*Enumeration · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandqueuetype>

Designates the queue type for a new input/output command queue.

## Declaration

```swift
enum MTLIOCommandQueueType
```

## Topics

### I/O command queue types
- [MTLIOCommandQueueType.concurrent](https://developer.apple.com/documentation/metal/mtliocommandqueuetype/concurrent) — Sets a new input/output command queue’s type to a queue that runs commands concurrently.
- [MTLIOCommandQueueType.serial](https://developer.apple.com/documentation/metal/mtliocommandqueuetype/serial) — Sets a new input/output command queue’s type to a queue that runs commands serially.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtliocommandqueuetype/init(rawvalue:))

## See also

### I/O command queues
- [MTLIOCommandQueue](https://developer.apple.com/documentation/metal/mtliocommandqueue) — A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
- [MTLIOCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor) — A configuration template you use to create a new input/output command queue.
- [MTLIOPriority](https://developer.apple.com/documentation/metal/mtliopriority) — Designates the priority for a new input/output command queue.
- [MTLIOScratchBufferAllocator](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator) — A protocol your app implements to provide scratch memory to an input/output command queue.
- [MTLIOScratchBuffer](https://developer.apple.com/documentation/metal/mtlioscratchbuffer) — A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.
