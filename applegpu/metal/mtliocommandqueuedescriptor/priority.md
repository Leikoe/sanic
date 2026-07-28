# priority

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/priority>

Configures the priority for a new input/output command queue.

## Declaration

```swift
var priority: MTLIOPriority { get set }
```

## See also

### Configuring the input/output command queue
- [type](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/type) — Configures the queue type for a new input/output command queue.
- [maxCommandsInFlight](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandsinflight) — Sets the largest number of individual commands that an input/output command queue can run at a time.
- [maxCommandBufferCount](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandbuffercount) — Sets the largest number of outstanding input/output command buffers a queue can have at any point in time.
