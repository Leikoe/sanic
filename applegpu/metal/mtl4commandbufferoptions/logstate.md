# logState

*Instance Property · iOS 26.0, iPadOS 26.0, Mac Catalyst 26.0, macOS 26.0, tvOS 26.0, visionOS 26.0*

<https://developer.apple.com/documentation/metal/mtl4commandbufferoptions/logstate>

Contains information related to shader logging.

## Declaration

```swift
var logState: (any MTLLogState)? { get set }
```

## Discussion

To enable shader logging, call [beginCommandBuffer(allocator:options:)](https://developer.apple.com/documentation/metal/mtl4commandbuffer/begincommandbuffer(allocator:options:)) with an instance of [MTL4CommandBufferOptions](https://developer.apple.com/documentation/metal/mtl4commandbufferoptions) that contains a non-`nil` [MTLLogState](https://developer.apple.com/documentation/metal/mtllogstate) instance in this property.

Shader functions log messages until the command buffer ends.
