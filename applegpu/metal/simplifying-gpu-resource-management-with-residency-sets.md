# Simplifying GPU resource management with residency sets

*Article*

<https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets>

Organize your resources into groups and influence when they become accessible to the GPU.

## Overview

Metal apps typically create resources, such as textures and buffers, so that their shaders can work with data as they run on the GPU. These resources need to be in memory that’s accessible to the GPU, or *resident*, so the shaders can access their data.

A *residency set* is one way you tell Metal which resources your app needs to make resident. You do this by creating [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) instances, managing which resource *allocations* they contain, and attaching them to command buffers or command queues. Resource allocation types conform to the [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation) protocol, including [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap).

The other way to tell Metal which resources it needs to make resident is by calling a command encoder’s methods. However, these methods can impact an app’s runtime performance because each call incurs some CPU overhead. Additionally, Metal makes those resources resident right after your app commits the command buffer, which can delay when the GPU starts working on it. This overhead adds up as the number of resources increases, especially in apps that use many resources for each frame, such as games.

Residency sets help you mitigate these performance issues and delays. With a residency set, your app can:

- Add multiple allocations with less CPU overhead than with a command encoder’s methods

- Make its allocations resident at the same time

- Request that Metal make its resources resident ahead of time

- Keep allocations resident indefinitely

- Remove all allocations, or a selection of them, which Metal marks as candidates that it can make nonresident, if necessary

You can attach each residency set to a command buffer or an entire command queue. Attaching a residency set to a command buffer removes the need to tell each of its command encoders which resources they need to use. Similarly, attaching a residency set to a command queue removes the need to attach that residency set to each of its command buffers.

### Make a residency set and add allocations to it

Create a residency set by configuring an [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) instance and passing it to the [makeResidencySet(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeresidencyset(descriptor:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice).

```swift
let setDescriptor = MTLResidencySetDescriptor()
setDescriptor.label = "Primary residency set"
setDescriptor.initialCapacity = 42

let residencySet = try device.makeResidencySet(descriptor: setDescriptor)
```

```objective-c
MTLResidencySetDescriptor *setDescriptor;
setDescriptor = [[MTLResidencySetDescriptor alloc] init];
setDescriptor.label = @"Primary residency set";
setDescriptor.initialCapacity = 42;

NSError *error;
id<MTLResidencySet> residencySet;
residencySet = [device newResidencySetWithDescriptor:setDescriptor
                                               error:&error];
```

Add an individual allocation to the [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) instance by calling its [addAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)) method, or add multiple allocations with its [addAllocations(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocations(_:)) method.

```swift
let residencySet = try device.makeResidencySet(descriptor: setDescriptor)

residencySet.addAllocation(buffer0)
residencySet.addAllocation(buffer1)
residencySet.addAllocation(texture0)
residencySet.addAllocation(texture1)
residencySet.addAllocation(heap)

let allocations = [buffer2,
                   texture2,
                   argumentBufferHeap,
                   textureHeap]

residencySet.addAllocations(allocations)
```

```objective-c
[residencySet addAllocation:buffer0];
[residencySet addAllocation:buffer1];
[residencySet addAllocation:texture0];
[residencySet addAllocation:texture1];
[residencySet addAllocation:heap];

id<MTLAllocation> allocations[] = {
    buffer2,
    texture2,
    argumentBufferHeap,
    textureHeap
};

[residencySet addAllocations:allocations
                       count:4];
```

A residency set handles redundant allocations by ignoring instances that already have an entry in the set.

> **Important:**
>  Adding a resource allocation that originates from an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) to a residency set makes that entire heap resident.

Finalize and apply the pending changes to the residency set by calling its [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method.

```swift
residencySet.commit()
```

```objective-c
[residencySet commit];
```

See [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) for information about working with residency sets, including:

- Inspecting current allocations

- Adding and removing allocations over time

- Accounting for resource hazards

### Attach a residency set to a command buffer

Connect an [MTLCommandBuffer](https://developer.apple.com/documentation/metal/mtlcommandbuffer) instance to a residency set’s resource allocations by attaching the set to the command buffer with the [useResidencySet(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/useresidencyset(_:)) or [useResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandbuffer/useresidencysets(_:)) method. Every command buffer can maintain a list of up to 32 different residency sets.

```swift
commandBuffer.useResidencySet(residencySet)
```

```objective-c
[commandBuffer useResidencySet:residencySet];
```

Metal makes the allocations in the set resident before the GPU runs the passes in the command buffer. This includes all resources that come from an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) allocation that’s in the residency set.

You don’t need to call the following methods for any allocation in a residency set that you associate with the command buffer:

| [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) | [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) |
|---|---|
| [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)) | [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) |
| [useResources(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:stages:)) | [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresources(_:usage:)) |
| [useHeap(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:stages:)) | [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)) |
| [useHeaps(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheaps(_:stages:)) | [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheaps(_:)) |

Attaching a residency set to a command buffer takes less CPU runtime and overhead than calling these methods for each encoder within a command buffer.

### Attach a residency set to a command queue and its command buffers

Connect an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) instance to a residency set’s resource allocations by attaching the set to the queue with its [addResidencySet(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/addresidencyset(_:)) or [addResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/addresidencysets(_:)) method. Every command queue can maintain a list of up to 32 different residency sets.

```swift
commandQueue.addResidencySet(residencySet)
```

```objective-c
[commandQueue addResidencySet:residencySet];
```

When your app calls a command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method, Metal automatically attaches the owning queue’s current residency sets to the command buffer.

> **Tip:**
>  Attach a residency set to a command queue for resources the GPU needs access to frequently, or for the lifetime of your app.

Attaching a residency set to a command queue is more efficient than attaching that residency set to multiple command buffers from that queue.

### Detach a residency set from a command queue

When your command queue doesn’t need the resources of a residency set, disconnect it from the queue by calling the [removeResidencySet(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/removeresidencyset(_:)) or [removeResidencySets(_:)](https://developer.apple.com/documentation/metal/mtlcommandqueue/removeresidencysets(_:)) method.

```swift
commandQueue.removeResidencySet(residencySet)
```

```objective-c
[commandQueue removeResidencySet:residencySet];
```

The residency set remains attached to any of the queue’s command buffers already in-flight with a status equal to [MTLCommandBufferStatus.committed](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/committed) or [MTLCommandBufferStatus.scheduled](https://developer.apple.com/documentation/metal/mtlcommandbufferstatus/scheduled).

### Request residency ahead of time

To make allocations in a residency set resident (for allocations that aren’t already resident), the Metal framework needs to do some work on the CPU. By default, Metal does this work when you call the [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method of the first command buffer that’s using the residency set. Making the allocations resident at this time can delay the graphics driver from submitting the command buffer to the GPU.

To help minimize the time between committing a command buffer and when the GPU starts working on it, ask Metal to do the work beforehand. You do this by calling a residency set’s [requestResidency()](https://developer.apple.com/documentation/metal/mtlresidencyset/requestresidency()) method.

```swift
residencySet.requestResidency()
```

```objective-c
[residencySet requestResidency];
```

Call this method at any time before you commit the first command buffer that relies on the allocations in the residency set. This can be any noncritical moment when your app can afford the CPU time the framework needs to prepare the applicable allocations for residency. For example, you can call this method at launch or during an app state change.

> **Note:**
>  The [requestResidency()](https://developer.apple.com/documentation/metal/mtlresidencyset/requestresidency()) method may postpone some of the necessary steps to make allocations resident in scenarios where other apps have competing memory needs.

### Conclude residency for the resources

When your app no longer needs a residency set’s allocations to be accessible to the GPU, call the [endResidency()](https://developer.apple.com/documentation/metal/mtlresidencyset/endresidency()) method, which effectively releases them.

```swift
residencySet.endResidency()
```

```objective-c
[residencySet endResidency];
```

The method tells Metal that it can reuse the memory backing that residency set’s allocations for your app’s other residency sets, or for another app.

## See also

### Residency sets
- [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) — A collection of resource allocations that can move in and out of resident memory.
- [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) — A configuration that customizes the behavior for a residency set.
