# waitUntilScheduled()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilscheduled()>

Blocks the current thread until the command queue schedules the buffer.

## Declaration

```swift
func waitUntilScheduled()
```

## Discussion

This method returns after the following events:

- The command queue *schedules* (see [status](https://developer.apple.com/documentation/metal/mtlcommandbuffer/status) and [MTLCommandBufferStatus.scheduled](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled)) the command buffer to run on the GPU.

- The command buffer invokes all the completion handlers your app submits with [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)).

Use the [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) method to check for completion of the scheduled work.

## See also

### Waiting for state changes
- [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) — Blocks the current thread until the GPU finishes executing the command buffer and all of its completion handlers.
