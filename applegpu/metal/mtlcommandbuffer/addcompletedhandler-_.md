# addCompletedHandler(_:)

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)>

Registers a completion handler the GPU device calls immediately after the GPU finishes running the commands in the command buffer.

## Declaration

```swift
func addCompletedHandler(_ block: @escaping MTLCommandBufferHandler)
```

## Parameters

- **block** — A Swift closure or an Objective-C block that Metal calls after the GPU finishes running the commands in the command buffer.

## Discussion

You can register one or more completion handlers for the same command buffer. The GPU device’s driver (on the CPU) calls the completion handlers after the GPU finishes executing the command buffer.

> **Important:**
>  You can only call this method before calling the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method.

For example, you can use the command buffer’s [gpuEndTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpuendtime) and [gpuStartTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpustarttime) properties to calculate how much time the GPU spends running the command buffer.

```swift
commandBuffer.addCompletedHandler { commandBuffer in
    let start = commandBuffer.gpuStartTime
    let end = commandBuffer.gpuEndTime

    let gpuRuntimeDuration = end - start

    /* ... */
}
```

```objective-c
[commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> commandBuffer) {
    CFTimeInterval start = commandBuffer.GPUStartTime;
    CFTimeInterval end = commandBuffer.GPUEndTime;

    CFTimeInterval gpuRuntimeDuration = end - start;

    /* ... */
}];
```

The completion handler is also a good place to check the [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) property to determine whether the GPU successfully completes the buffer’s commands. If the status is equal to [MTLCommandBufferStatus.error](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/error), you can investigate further by checking the [error](https://developer.apple.com/documentation/metal/mtlcommandbuffer/error) and log properties for more details about the issue. See [Command buffer debugging](https://developer.apple.com/documentation/metal/command-buffer-debugging) for more methods and properties that can help you isolate the issue.

> **Warning:**
>  Avoid calling the [insertDebugCaptureBoundary()](https://developer.apple.com/documentation/metal/mtlcommandqueue/insertdebugcaptureboundary()) method within the completion handler, which can cause a debug-time deadlock if you request GPU frame capture.

## See also

### Registering state change handlers
- [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) — Registers a completion handler the GPU device calls immediately after it schedules the command buffer to run on the GPU.
- [MTLCommandBufferHandler](https://developer.apple.com/documentation/metal/mtlcommandbufferhandler) — A completion handler signature a GPU device calls when it finishes scheduling a command buffer, or when the GPU finishes running it.
