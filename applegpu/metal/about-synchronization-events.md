# About synchronization events

*Article*

<https://developer.apple.com/documentation/metal/about-synchronization-events>

Synchronize access to resources in your app by signaling events.

## Overview

Use events to specify synchronization points in your app. For example, you might use an event to synchronize graphics rendering commands running on one command queue with compute processing being performed on another command queue.

Metal provides two different kinds of events:

- Nonshareable. [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) objects synchronize events within a single device object.

- Shareable. [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) objects synchronize events across multiple device objects, processors, or processes.

Shareable events have a higher overhead than nonshareable events. Don’t use [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) to synchronize events within a single device object; use [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) instead.

### Event signaling and waiting

An event contains a monotonically increasing unsigned 64-bit integer. An event starts with a value of `0`. You can either update the event’s value by *signaling* it, or block further execution by *waiting* on the event. Typically, you keep an integer value in your app for each event and increase its value each time you need to synchronize on the event. For example, you might increment the number each time you render a new frame of animation.

To signal a change to the event, call [encodeSignalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodesignalevent(_:value:)) on a command buffer, passing in the new value for this event. Metal signals the event after all scheduled commands prior to the event have finished, updating the event’s value if the new value is larger than its current value.

To wait for an event to be signaled, call [encodeWaitForEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodewaitforevent(_:value:)) on a command buffer, passing in the value to wait for. Commands that are after this event on the queue don’t run until the event’s value is at least as large as the value you provide.

> **Note:**
>  You can only encode events outside command encoder boundaries, not between encoded commands of a command encoder.

## See also

### Synchronizing with events
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device) — Use nonshareable events to synchronize your app’s work within a single device.
- [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes) — Use shareable events to synchronize your app’s work across multiple devices or processes.
- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu) — Use shareable events to synchronize your app’s work between a GPU and the CPU.
- [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) — A type that synchronizes memory operations to one or more resources within a single Metal device.
- [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) — A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.
- [MTLSharedEventHandle](https://developer.apple.com/documentation/metal/mtlsharedeventhandle) — An instance you use to recreate a shareable event.
- [MTLSharedEventListener](https://developer.apple.com/documentation/metal/mtlsharedeventlistener) — A listener for shareable event notifications.
- [MTLSharedEventNotificationBlock](https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock) — A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
