# MTLSharedEventHandle

*Class · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsharedeventhandle>

An instance you use to recreate a shareable event.

## Declaration

```swift
class MTLSharedEventHandle
```

## Overview

To create a `MTLSharedEventHandle` instance, call the [makeSharedEventHandle()](https://developer.apple.com/documentation/metal/mtlsharedevent/makesharedeventhandle()) method on an [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) instance. Use an XPC conection to pass a `MTLSharedEventHandle` instance to another process. To recreate the event, call the [makeSharedEvent(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent(handle:)) on an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

## Topics

### Identifying the shareable event handle
- [label](https://developer.apple.com/documentation/metal/mtlsharedeventhandle/label) — A string that identifies the shareable event.

### Initializers
- [init(coder:)](https://developer.apple.com/documentation/metal/mtlsharedeventhandle/init(coder:))

## See also

### Synchronizing with events
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [About synchronization events](https://developer.apple.com/documentation/metal/about-synchronization-events) — Synchronize access to resources in your app by signaling events.
- [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device) — Use nonshareable events to synchronize your app’s work within a single device.
- [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes) — Use shareable events to synchronize your app’s work across multiple devices or processes.
- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu) — Use shareable events to synchronize your app’s work between a GPU and the CPU.
- [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) — A type that synchronizes memory operations to one or more resources within a single Metal device.
- [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) — A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.
- [MTLSharedEventListener](https://developer.apple.com/documentation/metal/mtlsharedeventlistener) — A listener for shareable event notifications.
- [MTLSharedEventNotificationBlock](https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock) — A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
