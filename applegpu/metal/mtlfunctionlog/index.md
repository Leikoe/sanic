# MTLFunctionLog

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionlog>

A log entry a Metal device generates when the it runs a command buffer.

## Declaration

```swift
protocol MTLFunctionLog : NSObjectProtocol
```

## Topics

### Getting the log messsage
- [type](https://developer.apple.com/documentation/metal/mtlfunctionlog/type) — The type of message that was logged.
- [MTLFunctionLogType](https://developer.apple.com/documentation/metal/mtlfunctionlogtype) — Options for different kinds of function logs.

### Getting execution details
- [debugLocation](https://developer.apple.com/documentation/metal/mtlfunctionlog/debuglocation) — If known, the location of the logging command within a shader source file.
- [encoderLabel](https://developer.apple.com/documentation/metal/mtlfunctionlog/encoderlabel) — The label for the encoder that logged the message.
- [function](https://developer.apple.com/documentation/metal/mtlfunctionlog/function) — When known, the function object corresponding to the logged message.
- [MTLFunctionLogDebugLocation](https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation) — The source code that logged a debug message.

## See also

### Shader logs
- [MTLLogContainer](https://developer.apple.com/documentation/metal/mtllogcontainer-swift.struct) — A collection of logged messages, created when a Metal device runs a command buffer.
