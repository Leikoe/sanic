# MTLSharedEventNotificationBlock

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock>

A block of code invoked after a shareable event’s signal value equals or exceeds a given value.

## Declaration

```swift
typealias MTLSharedEventNotificationBlock = @Sendable (any MTLSharedEvent, UInt64) -> Void
```

## See also

### Synchronizing with events
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [About synchronization events](https://developer.apple.com/documentation/metal/about-synchronization-events) — Synchronize access to resources in your app by signaling events.
- [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device) — Use nonshareable events to synchronize your app’s work within a single device.
- [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes) — Use shareable events to synchronize your app’s work across multiple devices or processes.
- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu) — Use shareable events to synchronize your app’s work between a GPU and the CPU.
- [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) — A type that synchronizes memory operations to one or more resources within a single Metal device.
- [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) — A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.
- [MTLSharedEventHandle](https://developer.apple.com/documentation/metal/mtlsharedeventhandle) — An instance you use to recreate a shareable event.
- [MTLSharedEventListener](https://developer.apple.com/documentation/metal/mtlsharedeventlistener) — A listener for shareable event notifications.
