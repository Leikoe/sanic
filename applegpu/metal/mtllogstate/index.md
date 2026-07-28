# MTLLogState

*Protocol · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtllogstate>

A container for shader log messages.

## Declaration

```swift
protocol MTLLogState : NSObjectProtocol, Sendable
```

## Overview

Create an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) or [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) with a log state to hold messages logged from shaders. Attach a log state to a command buffer by assigning it to the command buffer descriptor’s [logState](https://developer.apple.com/documentation/metal/mtlcommandbufferdescriptor/logstate). Similarly, to attach a log state to a command queue, use the command queue descriptor’s [logState](https://developer.apple.com/documentation/metal/mtlcommandqueuedescriptor/logstate).

When you attach a log state to a command queue, the command queue shares the log state with all the command buffers it creates. If you attach different log states to a command buffer and command queue, then the system uses the state attached to the command buffer.

Because logging incurs an overhead, regardless of whether the system prints messages, you need to explicitly enable logging with [enableLogging](https://developer.apple.com/documentation/metal/mtlcompileoptions/enablelogging).

## Topics

### Instance Methods
- [addLogHandler(_:)](https://developer.apple.com/documentation/metal/mtllogstate/addloghandler(_:)) — Adds a log handler to customize the presentation of shader logging.

## See also

### Shader logging
- [MTLLogStateDescriptor](https://developer.apple.com/documentation/metal/mtllogstatedescriptor) — An interface that represents a log state configuration.
