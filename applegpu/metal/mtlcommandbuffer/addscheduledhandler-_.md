# addScheduledHandler(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)>

Registers a completion handler the GPU device calls immediately after it schedules the command buffer to run on the GPU.

## Declaration

```swift
func addScheduledHandler(_ block: @escaping MTLCommandBufferHandler)
```

## Parameters

- **block** — A Swift closure or an Objective-C block that Metal calls after it schedules the command buffer to run on the GPU.

## Discussion

You can register one or more scheduling completion handlers for the same command buffer. The GPU device’s driver (on the CPU) calls the completion handlers after it finishes scheduling the command buffer to run on the GPU.

> **Important:**
>  You can only call this method before calling the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method.

The GPU device schedules each command buffer — along with tasks from other command buffers — after it identifies the command buffer’s dependencies. At that time, the GPU device sets the command buffer’s status to [MTLCommandBufferStatus.scheduled](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled) and calls your completion handler.

> **Note:**
>  The command buffer’s [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) property may be equal to another (larger) value by the time your completion handler runs, including [MTLCommandBufferStatus.completed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/completed).

You can use the command buffer’s [kernelEndTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelendtime) and [kernelStartTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelstarttime) properties to calculate how much time the CPU spends scheduling the command buffer.

```swift
commandBuffer.addScheduledHandler { commandBuffer in
    let start = commandBuffer.kernelStartTime
    let end = commandBuffer.kernelEndTime

    let scheduleDuration = end - start

    /* ... */
}
```

```objective-c
[commandBuffer addScheduledHandler:^(id<MTLCommandBuffer> commandBuffer) {
    CFTimeInterval start = commandBuffer.kernelStartTime;
    CFTimeInterval end = commandBuffer.kernelEndTime;

    CFTimeInterval scheduleDuration = end - start;

    /* ... */
}];

```

## See also

### Registering state change handlers
- [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) — Registers a completion handler the GPU device calls immediately after the GPU finishes running the commands in the command buffer.
- [MTLCommandBufferHandler](https://developer.apple.com/documentation/metal/mtlcommandbufferhandler) — A completion handler signature a GPU device calls when it finishes scheduling a command buffer, or when the GPU finishes running it.
