# tryCancel()

*Instance Method · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandbuffer/trycancel()>

Submits a request to abandon a command buffer the queue is currently running.

## Declaration

```swift
func tryCancel()
```

## Discussion

Check the command buffer’s [status](https://developer.apple.com/documentation/metal/mtliocommandbuffer/status) property after it completes, either after [waitUntilCompleted()](https://developer.apple.com/documentation/metal/mtliocommandbuffer/waituntilcompleted()) or in one of your completion handlers (see [addCompletedHandler(_:)](https://developer.apple.com/documentation/metal/mtliocommandbuffer/addcompletedhandler(_:))).
