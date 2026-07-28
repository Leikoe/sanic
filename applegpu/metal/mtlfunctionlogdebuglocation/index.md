# MTLFunctionLogDebugLocation

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation>

The source code that logged a debug message.

## Declaration

```swift
protocol MTLFunctionLogDebugLocation : NSObjectProtocol
```

## Topics

### Inspecting the location details
- [functionName](https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation/functionname) — The name of the shader function.
- [url](https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation/url) — The URL of the file that contains the shader function.
- [line](https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation/line) — The line that the log message appears on.
- [column](https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation/column) — The column where the log message appears.

## See also

### Getting execution details
- [debugLocation](https://developer.apple.com/documentation/metal/mtlfunctionlog/debuglocation) — If known, the location of the logging command within a shader source file.
- [encoderLabel](https://developer.apple.com/documentation/metal/mtlfunctionlog/encoderlabel) — The label for the encoder that logged the message.
- [function](https://developer.apple.com/documentation/metal/mtlfunctionlog/function) — When known, the function object corresponding to the logged message.
