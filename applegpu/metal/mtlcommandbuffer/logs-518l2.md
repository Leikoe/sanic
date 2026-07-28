# logs

*Instance Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/logs-518l2>

The messages the command buffer records as the GPU runs its commands.

## Declaration

```swift
var logs: MTLLogContainer { get }
```

## Discussion

The value of this property is valid only after the command buffer finishes executing.
