# MTLCommandBufferHandler

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlcommandbufferhandler>

A completion handler signature a GPU device calls when it finishes scheduling a command buffer, or when the GPU finishes running it.

## Declaration

```swift
typealias MTLCommandBufferHandler = @Sendable (any MTLCommandBuffer) -> Void
```

## Parameters

- **commandBuffer** — The [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance that’s invoking the completion handler.

## Discussion

The [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) type uses this signature in its methods that register your completion handlers, including [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) and [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)).

## See also

### Registering state change handlers
- [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) — Registers a completion handler the GPU device calls immediately after it schedules the command buffer to run on the GPU.
- [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) — Registers a completion handler the GPU device calls immediately after the GPU finishes running the commands in the command buffer.
