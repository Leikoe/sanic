# level

*Instance Property · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtllogstatedescriptor/level>

The minimum level of messages that the shader can log.

## Declaration

```swift
var level: MTLLogLevel { get set }
```

## Discussion

The default value is [MTLLogLevel.debug](https://developer.apple.com/documentation/metal/mtlloglevel/debug).

Use this value to limit which logs from your shader the log state stores. The log state doesn’t store messages at a lower level. Increase the level to reduce verbosity of logging.

## See also

### Instance properties
- [bufferSize](https://developer.apple.com/documentation/metal/mtllogstatedescriptor/buffersize) — The size of the internal buffer the log state uses, specified in bytes.
