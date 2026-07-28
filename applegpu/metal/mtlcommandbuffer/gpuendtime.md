# gpuEndTime

*Instance Property · iOS 10.3, iPadOS 10.3, Mac Catalyst 13.0, macOS 10.15, tvOS 10.2, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpuendtime>

The host time, in seconds, when the GPU finishes execution of the command buffer.

## Declaration

```swift
var gpuEndTime: CFTimeInterval { get }
```

## Discussion

You can calculate how much time the GPU spends running a command buffer by subtracting [gpuStartTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpustarttime) from this value. Both values are relative to system mach time.

The GPU start and end times remain `0.0` until the GPU finishes running the command buffer. Check this value after the [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()) method returns, or within a completion handler passed to the [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/addcompletedhandler(_:)) method.

## See also

### Checking execution times on the GPU
- [gpuStartTime](https://developer.apple.com/documentation/metal/mtlcommandbuffer/gpustarttime) — The host time, in seconds, when the GPU starts command buffer execution.
