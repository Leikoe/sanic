# waitUntilCompleted()

*Instance Method · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilcompleted()>

Blocks the current thread until the GPU finishes executing the command buffer and all of its completion handlers.

## Declaration

```swift
func waitUntilCompleted()
```

## See also

### Waiting for state changes
- [waitUntilScheduled()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/waituntilscheduled()) — Blocks the current thread until the command queue schedules the buffer.
