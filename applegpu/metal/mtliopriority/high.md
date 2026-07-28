# MTLIOPriority.high

*Case · iOS 16.0, iPadOS 16.0, Mac Catalyst 16.0, macOS 13.0, tvOS 16.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtliopriority/high>

Sets a new input/output command queue’s priority to a high priority.

## Declaration

```swift
case high
```

## Discussion

Create a command queue with a high priority to load important assets or those your app needs quickly. For example, a game that plays sound effects that match its animations can load its audio assets with low latency with a high priority queue.

## See also

### I/O command queue priorities
- [MTLIOPriority.normal](https://developer.apple.com/documentation/metal/mtliopriority/normal) — Designates the normal priority for a new input/output command queue.
- [MTLIOPriority.low](https://developer.apple.com/documentation/metal/mtliopriority/low) — Designates the low priority for a new input/output command queue.
