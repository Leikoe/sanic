# kernelStartTime

*Instance Property · iOS 10.3, iPadOS 10.3, Mac Catalyst 13.0, macOS 10.15, tvOS 10.2, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelstarttime>

The host time, in seconds, when the CPU begins to schedule the command buffer.

## Declaration

```swift
var kernelStartTime: CFTimeInterval { get }
```

## Discussion

You can calculate how much time the kernel spends scheduling a command buffer by subtracting this value from [kernelEndTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelendtime).

The kernel start and end times remain `0.0` until the GPU driver (on the CPU) schedules the command buffer to run on the GPU. Apps typically use these values after the [waitUntilScheduled()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilscheduled()) method returns, or within a completion handler (see [addScheduledHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addscheduledhandler(_:)) and [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:))).

## See also

### Checking scheduling times on the CPU
- [kernelEndTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/kernelendtime) — The host time, in seconds, when the CPU finishes scheduling the command buffer.
