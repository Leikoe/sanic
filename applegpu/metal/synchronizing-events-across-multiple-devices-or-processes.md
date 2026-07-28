# Synchronizing events across multiple devices or processes

*Article*

<https://developer.apple.com/documentation/metal/synchronizing-events-across-multiple-devices-or-processes>

Use shareable events to synchronize your app’s work across multiple devices or processes.

## Overview

The following figure and code show a shareable event that synchronizes graphics rendering on one device with compute processing on another.

![image](https://docs-assets.developer.apple.com/published/8f7ba4dbc1f70a4e848435f95ff8bac8/synchronizing-events-across-multiple-devices-or-processes-1%402x.png)

```swift
func setupMultipleDeviceEvent() {
    // Shareable event
    sharedEvent = deviceA.makeSharedEvent()
    
    // Built-in GPU command queue
    commandQueueA = deviceA.makeCommandQueue()
    
    // External GPU command queue
    commandQueueB = deviceB.makeCommandQueue()
}

func renderFrame() {
    guard
        let sharedEvent = sharedEvent,
        let commandBufferA = commandQueueA?.makeCommandBuffer(),
        let commandBufferB = commandQueueB?.makeCommandBuffer()
        else { return }
    
    // Device A (Graphics Rendering)
    /* Encode first render pass */
    commandBufferA.encodeSignalEvent(sharedEvent, value: 1)
    /* Encode second render pass */
    commandBufferA.encodeWaitForEvent(sharedEvent, value: 2)
    /* Encode third render pass */
    commandBufferA.commit()
    
    // Device B (Compute Processing)
    /* Encode first compute pass */
    commandBufferB.encodeWaitForEvent(sharedEvent, value: 1)
    /* Encode second compute pass  */
    commandBufferB.encodeSignalEvent(sharedEvent, value: 2)
    /* Encode third compute pass */
    commandBufferB.commit()
}
```

```objective-c
- (void)setupMultipleDeviceEvent
{
    // Shareable event
    _sharedEvent = [_deviceA newSharedEvent];
    
    // Built-in GPU command queue
    _commandQueueA = [_deviceA newCommandQueue];
    
    // External GPU command queue
    _commandQueueB = [_deviceB newCommandQueue];
}

- (void)renderFrame
{
    // Device A (Graphics Rendering)
    id<MTLCommandBuffer> commandBufferA = [_commandQueueA commandBuffer];
    /* Encode first render pass */
    [commandBufferA encodeSignalEvent:_sharedEvent value:1];
    /* Encode second render pass  */
    [commandBufferA encodeWaitForEvent:_sharedEvent value:2];
    /* Encode third render pass  */
    [commandBufferA commit];
    
    // Device B (Compute Processing)
    id<MTLCommandBuffer> commandBufferB = [_commandQueueB commandBuffer];
    /* Encode first compute pass */
    [commandBufferB encodeWaitForEvent:_sharedEvent value:1];
    /* Encode second compute pass */
    [commandBufferB encodeSignalEvent:_sharedEvent value:2];
    /* Encode third compute pass */
    [commandBufferB commit];
}
```

During setup, the code creates a shareable event ([MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent)) and command queues on two different devices. Like the example shown in [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device), it encodes render commands onto the first queue and compute commands on the second queue.

You call the same methods when signaling and waiting on shared events as you do when working with events on a single device. The only difference is that the queues are associated with different devices and the event being used to synchronize access is a shared event.

The code shown above assumes you’ve created each resource on both device objects, and each pair of resources share a single allocation of memory. This strategy means that change made by one device object are visible to the other device object. For an example of how to do this, see [Selecting device objects for compute processing](https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing).

## See also

### Synchronizing with events
- [Implementing a multistage image filter using heaps and events](https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-events) — Use events to synchronize access to resources allocated on a heap.
- [About synchronization events](https://developer.apple.com/documentation/metal/about-synchronization-events) — Synchronize access to resources in your app by signaling events.
- [Synchronizing events within a single device](https://developer.apple.com/documentation/metal/synchronizing-events-within-a-single-device) — Use nonshareable events to synchronize your app’s work within a single device.
- [Synchronizing events between a GPU and the CPU](https://developer.apple.com/documentation/metal/synchronizing-events-between-a-gpu-and-the-cpu) — Use shareable events to synchronize your app’s work between a GPU and the CPU.
- [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) — A type that synchronizes memory operations to one or more resources within a single Metal device.
- [MTLSharedEvent](https://developer.apple.com/documentation/metal/mtlsharedevent) — A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.
- [MTLSharedEventHandle](https://developer.apple.com/documentation/metal/mtlsharedeventhandle) — An instance you use to recreate a shareable event.
- [MTLSharedEventListener](https://developer.apple.com/documentation/metal/mtlsharedeventlistener) — A listener for shareable event notifications.
- [MTLSharedEventNotificationBlock](https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock) — A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
