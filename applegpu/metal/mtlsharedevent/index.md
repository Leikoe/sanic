# MTLSharedEvent

*Protocol · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlsharedevent>

A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.

## Declaration

```swift
protocol MTLSharedEvent : MTLEvent
```

## Overview

The [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) protocol inherits the [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) protocol. An event can only synchronize memory operations that run on a single Metal device. A shared event can synchronize memory operations across multiple Metal devices and the CPU. Shared events work anywhere you can work with a regular event.

> **Tip:**
> Start with an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) instance until you need to synchronize work with a task that runs on the CPU or another Metal device, because an [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) can add overhead that may affect your app’s performance.

Create an [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) by calling the [makeSharedEvent()](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent()) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

To pass this event to another process:

1. Create a handle to the shared event by calling the [makeSharedEventHandle()](https://developer.apple.com/documentation/metal/mtlsharedevent/makesharedeventhandle()) method.

2. Transfer the handle to another process with XPC.

3. From the other process, call the [makeSharedEvent(handle:)](https://developer.apple.com/documentation/metal/mtldevice/makesharedevent(handle:)) method.

For more information about shared events and synchronizing memory operations to resources, see:

- [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes)

- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu).

- [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization)

## Topics

### Synchronizing a shareable event
- [signaledValue](https://developer.apple.com/documentation/metal/mtlsharedevent/signaledvalue) — The current signal value for the shareable event.
- [notify(_:atValue:block:)](https://developer.apple.com/documentation/metal/mtlsharedevent/notify(_:atvalue:block:)) — Schedules a notification handler to be called after the shareable event’s signal value equals or exceeds a given value.

### Creating a shared event handle
- [makeSharedEventHandle()](https://developer.apple.com/documentation/metal/mtlsharedevent/makesharedeventhandle()) — Creates a new shareable event handle.

### Instance Methods
- [valueSignaled(_:)](https://developer.apple.com/documentation/metal/mtlsharedevent/valuesignaled(_:))
- [wait(untilSignaledValue:timeoutMS:)](https://developer.apple.com/documentation/metal/mtlsharedevent/wait(untilsignaledvalue:timeoutms:))

## See also

### Synchronizing with events
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [About synchronization events](https://developer.apple.com/documentation/metal/about-synchronization-events) — Synchronize access to resources in your app by signaling events.
- [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device) — Use nonshareable events to synchronize your app’s work within a single device.
- [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes) — Use shareable events to synchronize your app’s work across multiple devices or processes.
- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu) — Use shareable events to synchronize your app’s work between a GPU and the CPU.
- [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) — A type that synchronizes memory operations to one or more resources within a single Metal device.
- [MTLSharedEventHandle](https://developer.apple.com/documentation/metal/mtlsharedeventhandle) — An instance you use to recreate a shareable event.
- [MTLSharedEventListener](https://developer.apple.com/documentation/metal/mtlsharedeventlistener) — A listener for shareable event notifications.
- [MTLSharedEventNotificationBlock](https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock) — A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
