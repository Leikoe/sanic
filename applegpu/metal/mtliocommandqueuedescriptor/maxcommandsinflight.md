# maxCommandsInFlight

*Instance Property · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandsinflight>

Sets the largest number of individual commands that an input/output command queue can run at a time.

## Declaration

```swift
var maxCommandsInFlight: Int { get set }
```

## Discussion

Set to `0` to instruct Metal to select an appropriate value for you — based on the system’s available memory.

## See also

### Configuring the input/output command queue
- [priority](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/priority) — Configures the priority for a new input/output command queue.
- [type](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/type) — Configures the queue type for a new input/output command queue.
- [maxCommandBufferCount](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor/maxcommandbuffercount) — Sets the largest number of outstanding input/output command buffers a queue can have at any point in time.
