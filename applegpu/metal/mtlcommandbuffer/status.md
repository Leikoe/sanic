# status

*Instance Property · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/status>

The command buffer’s current state.

## Declaration

```swift
var status: MTLCommandBufferStatus { get }
```

## Discussion

Each command buffer can be in any one of the following states:

| State | Meaning |
|---|---|
| [MTLCommandBufferStatus.notEnqueued](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/notenqueued) | A command buffer’s initial state, which indicates its command queue isn’t reserving a place for it. ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) You can modify a command buffer in this state by encoding commands to it, or by adding a state change handler. |
| [MTLCommandBufferStatus.enqueued](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/enqueued) | A command buffer’s second state, which indicates its command queue is reserving a place for it. ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) You can modify a command buffer in this state by encoding commands to it, or by adding a state change handler. |
| [MTLCommandBufferStatus.committed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/committed) | A command buffer’s third state, which indicates the command queue is preparing to schedule the command buffer by resolving its dependencies. ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) You can’t modify a command buffer in this state. |
| [MTLCommandBufferStatus.scheduled](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled) | A command buffer’s fourth state, which indicates the command buffer has its resources ready and is waiting for the GPU to run its commands. ![image](https://docs-assets.developer.apple.com/published/67dc4b07a8d84366d4cc0e812eb40b4a/spacer.png) You can’t modify a command buffer in this state. |
| [MTLCommandBufferStatus.completed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/completed) | A command buffer’s successful, final state, which indicates the GPU finished running the command buffer’s commands without any problems. |
| [MTLCommandBufferStatus.error](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/error) | A command buffer’s unsuccessful, final state, which indicates the GPU stopped running the buffer’s commands because of a runtime issue. |

The first two states ([MTLCommandBufferStatus.notEnqueued](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/notenqueued) and [MTLCommandBufferStatus.enqueued](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/enqueued)) both indicate that you can encode commands to the command buffer. You do this by creating an encoder that indirectly adds commands for a pass (see [Command encoder factory methods](https://developer.apple.com/documentation/metal/command-encoder-factory-methods)) to the command buffer. Command buffers also have some methods that directly encode commands between passes, such as [encodeSignalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodesignalevent(_:value:)) and [present(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/present(_:)).

Each command buffer’s state can only change to a state below it in the table, and ends its life cycle at either [MTLCommandBufferStatus.completed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/completed) or [MTLCommandBufferStatus.error](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/error).

## See also

### Troubleshooting a command buffer
- [MTLCommandBufferStatus](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus) — The discrete states for a command buffer that represent its life cycle stages.
- [Command buffer debugging](https://developer.apple.com/documentation/metal/command-buffer-debugging) — Properties and methods for programmatically debugging runtime issues with a command buffer.
