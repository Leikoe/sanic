# MTLLogStateDescriptor

*Class · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtllogstatedescriptor>

An interface that represents a log state configuration.

## Declaration

```swift
class MTLLogStateDescriptor
```

## Overview

Configure the descriptor to create an [MTLLogState](https://developer.apple.com/documentation/metal/mtllogstate) by calling [makeLogState(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makelogstate(descriptor:)).

If you’ve set the environment variables `MTL_LOG_BUFFER_SIZE` or `MTL_LOG_LEVEL`, then the system automatically enables logging. If any command buffer or command queue has an attached log state, then the system uses the log state’s settings instead of the environment variable values.

## Topics

### Instance properties
- [bufferSize](https://developer.apple.com/documentation/metal/mtllogstatedescriptor/buffersize) — The size of the internal buffer the log state uses, specified in bytes.
- [level](https://developer.apple.com/documentation/metal/mtllogstatedescriptor/level) — The minimum level of messages that the shader can log.

### Log levels
- [MTLLogLevel](https://developer.apple.com/documentation/metal/mtlloglevel) — The supported log levels for shader logging.

## See also

### Shader logging
- [MTLLogState](https://developer.apple.com/documentation/metal/mtllogstate) — A container for shader log messages.
