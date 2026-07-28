# MTLEvent

*Protocol · iOS 12.0, iPadOS 12.0, Mac Catalyst 13.1, macOS 10.14, tvOS 12.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlevent>

A type that synchronizes memory operations to one or more resources within a single Metal device.

## Declaration

```swift
protocol MTLEvent : NSObjectProtocol, Sendable
```

## Overview

Each event represents an unsigned 64-bit integer that starts with a value of `0`, which can only increase over time. Create an [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) by calling the [makeEvent()](https://developer.apple.com/documentation/metal/mtldevice/makeevent()) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance.

With an event, synchronize commands across a single Metal device by instructing it to wait before starting a workload, such as a compute pass, until another workload finishes. You do this by encoding signal and wait commands:

- Add a signal command after encoding the producing workload that one or more other workloads depend on.

- Add a wait command before encoding each consuming workload that depends on the producing workload.

The Metal device begins running any dependent workloads when the event equals or exceeds the value that the wait command is waiting for.

### Synchronize one producing workload with an event

When working with Metal 4 types, add wait and signal commands with an [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) instance by calling its methods:

- [waitForEvent(_:value:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/waitforevent(_:value:))

- [signalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtl4commandqueue/signalevent(_:value:))

Similarly for Metal 3 and earlier, add wait and signal commands with an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance by calling its methods:

- [encodeWaitForEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodewaitforevent(_:value:))

- [encodeSignalEvent(_:value:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/encodesignalevent(_:value:))

> **Important:**
>  You can signal an event only with a new value that’s greater than its current value.

When a Metal device reaches a wait command, it compares the event’s current value to the command’s target value. The device proceeds to the subsequent commands only when another command updates the event with a value that’s equal to or greater than the target value. For an example that synchronizes workloads on different queues within the same device with a single event instance, see [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device).

You can add signal and wait commands to any combination of [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) and [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instances that all belong to the same Metal device. Even though you encode a wait command before the signal command that unblocks it, minimize the time between when they run because wait commands can time out.

> **Important:**
>  Wait commands that time out in an [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue) unblock subsequent work in the queue, and wait commands that time out in an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) terminate the command buffer with the [MTLCommandBufferError.Code.timeout](https://developer.apple.com/documentation/metal/mtlcommandbuffererror-swift.struct/code/timeout) error code.

One event signal can unblock multiple workloads waiting for it. For example, if workload A needs to run before starting workloads B and C, the B and C workloads can wait for one event to reach a specific value, such as `0x42`. When workload A finishes, the next command can unblock workloads B and C by signaling that event with the value `0x42` or greater.

### Synchronize multiple producing workloads with an event for each

Multiple producing workloads can’t combine their signals with one event to unblock any dependent workloads. This is because an event’s signal method can only increase its value to a specific number, unlike a semaphore that can increment or decrement its current value by one. Instead, signal when each producing workload finishes by updating its own separate event. Dependent workloads can wait for the multiple events that correspond to the workloads they depend on.

> **Tip:**
> For workloads with complicated dependency chains, consider other access synchronization mechanisms that [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization) introduces.

## Topics

### Identifying the event
- [device](https://developer.apple.com/documentation/metal/mtlevent/device) — The device object that created the event.
- [label](https://developer.apple.com/documentation/metal/mtlevent/label) — A string that identifies the event.

## See also

### Synchronizing with events
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [About synchronization events](https://developer.apple.com/documentation/metal/about-synchronization-events) — Synchronize access to resources in your app by signaling events.
- [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device) — Use nonshareable events to synchronize your app’s work within a single device.
- [Synchronizing events across multiple devices or processes](https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes) — Use shareable events to synchronize your app’s work across multiple devices or processes.
- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu) — Use shareable events to synchronize your app’s work between a GPU and the CPU.
- [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) — A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.
- [MTLSharedEventHandle](https://developer.apple.com/documentation/metal/mtlsharedeventhandle) — An instance you use to recreate a shareable event.
- [MTLSharedEventListener](https://developer.apple.com/documentation/metal/mtlsharedeventlistener) — A listener for shareable event notifications.
- [MTLSharedEventNotificationBlock](https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock) — A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
