# MTLCommandEncoder

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandencoder>

An encoder that writes GPU commands into a command buffer.

## Declaration

```swift
protocol MTLCommandEncoder : NSObjectProtocol
```

## Overview

Don’t implement this protocol yourself; instead you call methods on an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance to create command encoders. Command encoder instances are lightweight instances that you re-create every time you need to send commands to the GPU.

There are many different kinds of command encoders, each providing a different set of commands that can be encoded into the buffer. A command encoder implements the [MTLCommandEncoder](https://developer.apple.com/documentation/metal/mtlcommandencoder) protocol and an additional protocol specific to the kind of encoder being created.

| Protocol | Task |
|---|---|
| [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) | Graphics rendering |
| [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) | Computation |
| [MTLBlitCommandEncoder](https://developer.apple.com/documentation/metal/mtlblitcommandencoder) | Memory management |
| [MTLParallelRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder) | Multiple graphics rendering tasks encoded in parallel. |

While a command encoder is active, it has the exclusive right to append commands to its command buffer. Once you finish encoding commands, call the [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) method to finish encoding the commands. To write further commands into the same command buffer, create a new command encoder.

You can call the [insertDebugSignpost(_:)](https://developer.apple.com/documentation/metal/mtlcommandencoder/insertdebugsignpost(_:)), [pushDebugGroup(_:)](https://developer.apple.com/documentation/metal/mtlcommandencoder/pushdebuggroup(_:)), and [popDebugGroup()](https://developer.apple.com/documentation/metal/mtlcommandencoder/popdebuggroup()) methods to put debug strings into the command buffer and to push or pop string labels used to identify groups of encoded commands. These methods don’t change the rendering or compute behavior of your app; the Xcode debugger uses them to organize your app’s rendering commands in a format that may provide insight into how your app works.

## Topics

### Ending command encoding
- [endEncoding()](https://developer.apple.com/documentation/metal/mtlcommandencoder/endencoding()) — Declares that all command generation from the encoder is completed.

### Annotating the command buffer with debug information
- [insertDebugSignpost(_:)](https://developer.apple.com/documentation/metal/mtlcommandencoder/insertdebugsignpost(_:)) — Inserts a debug string into the captured frame data.
- [pushDebugGroup(_:)](https://developer.apple.com/documentation/metal/mtlcommandencoder/pushdebuggroup(_:)) — Pushes a specific string onto a stack of debug group strings for the command encoder.
- [popDebugGroup()](https://developer.apple.com/documentation/metal/mtlcommandencoder/popdebuggroup()) — Pops the latest string off of a stack of debug group strings for the command encoder.

### Identifying the command encoder
- [device](https://developer.apple.com/documentation/metal/mtlcommandencoder/device) — The Metal device from which the command encoder was created.
- [label](https://developer.apple.com/documentation/metal/mtlcommandencoder/label) — A string that labels the command encoder.

### Instance Methods
- [barrier(afterQueueStages:beforeStages:)](https://developer.apple.com/documentation/metal/mtlcommandencoder/barrier(afterqueuestages:beforestages:)) — Encodes a consumer barrier on work you commit to the same command queue.

## See also

### Submitting work to a GPU with Metal
- [Setting up a command structure](https://developer.apple.com/documentation/metal/setting-up-a-command-structure) — Discover how Metal executes commands on a GPU.
- [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) — An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
- [MTLCommandQueueDescriptor](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor) — A configuration that customizes the behavior for a new command queue.
- [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) — A container that stores a sequence of GPU commands that you encode into it.
- [MTLCommandBufferDescriptor](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor) — A configuration that customizes the behavior for a new command buffer.
- [MTLCommandBufferError](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct) — The command buffer error codes that indicate why the GPU doesn’t finish executing a command buffer.
