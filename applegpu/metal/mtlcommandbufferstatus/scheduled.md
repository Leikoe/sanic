# MTLCommandBufferStatus.scheduled

*Case · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled>

A command buffer’s fourth state, which indicates the command buffer has its resources ready and is waiting for the GPU to run its commands.

## Declaration

```swift
case scheduled
```

## Discussion

See the [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) protocol’s [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) property for more information.

## See also

### Command buffer states
- [MTLCommandBufferStatus.notEnqueued](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/notenqueued) — A command buffer’s initial state, which indicates its command queue isn’t reserving a place for it.
- [MTLCommandBufferStatus.enqueued](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/enqueued) — A command buffer’s second state, which indicates its command queue is reserving a place for it.
- [MTLCommandBufferStatus.committed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/committed) — A command buffer’s third state, which indicates the command queue is preparing to schedule the command buffer by resolving its dependencies.
- [MTLCommandBufferStatus.completed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/completed) — A command buffer’s successful, final state, which indicates the GPU finished running the command buffer’s commands without any problems.
- [MTLCommandBufferStatus.error](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/error) — A command buffer’s unsuccessful, final state, which indicates the GPU stopped running the buffer’s commands because of a runtime issue.
