# MTLIOCommandBuffer

*Protocol · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer>

A command buffer that contains input/output commands that work with files in the file systems and Metal resources.

## Declaration

```swift
protocol MTLIOCommandBuffer : NSObjectProtocol
```

## Overview

Add commands an input/output command buffer to load assets from the file system directly into Metal resources. Your app can then use those resources with other commands it submits to [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue).

## Topics

### Loading assets
- [load(_:offset:size:sourceHandle:sourceHandleOffset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/load(_:offset:size:sourcehandle:sourcehandleoffset:)) — Encodes a command that loads data from a file handle into a GPU buffer.
- [load(_:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/load(_:slice:level:size:sourcebytesperrow:sourcebytesperimage:destinationorigin:sourcehandle:sourcehandleoffset:)) — Encodes a command that loads data from a file handle into a GPU texture.
- [loadBytes(_:size:sourceHandle:sourceHandleOffset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/loadbytes(_:size:sourcehandle:sourcehandleoffset:)) — Encodes a command that loads data from a file handle into CPU-accessible memory buffer.

### Adding a barrier
- [addBarrier()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/addbarrier()) — Encodes a barrier into the command buffer.

### Synchronizing a command buffer
- [signalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/signalevent(_:value:)) — Encodes a command that signals a shared event to other parts of your app.
- [waitForEvent(_:value:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/waitforevent(_:value:)) — Encodes a command that pauses the command buffer’s execution until another part of your app signals a shared event.

### Adding final commands
- [copyStatus(buffer:offset:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/copystatus(buffer:offset:)) — Encodes a command that writes the input/output command buffer’s status to a buffer.
- [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/addcompletedhandler(_:)) — Adds a closure that Metal calls immediately after the GPU finishes executing the commands in the input/output command buffer.

### Submitting a command buffer
- [commit()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/commit()) — Submits the command buffer to the queue for execution on the GPU.
- [enqueue()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/enqueue()) — Reserves a place for the input/output command buffer in the input/output command queue without committing the command buffer.

### Canceling a command buffer
- [tryCancel()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/trycancel()) — Submits a request to abandon a command buffer the queue is currently running.

### Waiting for a command buffer
- [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/waituntilcompleted()) — Blocks the current thread until the GPU finishes executing the input/output command buffer and all of its completion handlers.

### Checking the state of a command buffer
- [status](https://developer.apple.com/documentation/metal/mtliocommandbuffer/status) — Represents the state of the input/output command buffer.
- [error](https://developer.apple.com/documentation/metal/mtliocommandbuffer/error) — Stores the details of an error when the GPU experienced a problem with the input/output command buffer.

### Debugging a command buffer
- [label](https://developer.apple.com/documentation/metal/mtliocommandbuffer/label) — An optional name for the input/output command buffer.
- [pushDebugGroup(_:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/pushdebuggroup(_:)) — Sets the current name for this input/output command encoder by adding it to the top of the debug name stack.
- [popDebugGroup()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/popdebuggroup()) — Restores the previous name for this input/output command encoder by removing the top item of the debug name stack.

## See also

### I/O command buffers
- [MTLIOFileHandle](https://developer.apple.com/documentation/metal/mtliofilehandle) — Represents a raw or compressed file, such as a resource asset file in your app’s bundle.
- [MTLIOCommandBufferHandler](https://developer.apple.com/documentation/metal/mtliocommandbufferhandler) — A convenience type that defines the signature of an input/output command buffer’s completion handler.
- [MTLIOStatus](https://developer.apple.com/documentation/metal/mtliostatus) — Represents the state of an input/output command buffer.
- [MTLIOError.Code](https://developer.apple.com/documentation/metal/mtlioerror-swift.struct/code) — The error codes for creating an input/output file handle.
- [MTLIOErrorDomain](https://developer.apple.com/documentation/metal/mtlioerrordomain) — The domain for input/output command queue errors.
