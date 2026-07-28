# MTLResidencySet

*Protocol · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlresidencyset>

A collection of resource allocations that can move in and out of resident memory.

## Declaration

```swift
protocol MTLResidencySet : NSObjectProtocol
```

## Overview

Residency sets are a way you can tell Metal which resource allocations, such as buffers, textures, and heaps, to make *resident*, or GPU-accessible. Adding allocations to a residency set requires less overhead than the equivalent methods of a command encoder. Residency sets also give you more control when Metal makes their allocations resident, and for how long they remain resident. However, residency sets don’t track hazards, so you need to account for hazards with fences and events.

You can change which [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation) instances are in a residency set at any time by:

1. Staging additions and removals with the [addAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)) and [removeAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocation(_:)) methods, respectively, or with their sibling methods

2. Applying staged changes by calling the residency set’s [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method

Metal doesn’t synchronize the state of the residency set between the CPU and the GPU. This means you can add resource allocations to the set while the GPU is actively running a command buffer that’s accessing them.

> **Important:**
>  If there’s a resource in a residency set that the GPU no longer needs access to, you can remove that resource from the residency set, even while the GPU is actively accessing other resources from the same residency set.

Metal makes the union of all residency sets’ allocations resident. This means each resource allocation, such as a buffer, can have an entry in multiple residency sets at the same time. Removing an allocation from one residency set doesn’t affect its residency if it also has an entry in another residency set. So you can remove an entire residency set from a command queue and only remove the allocations from residency that are unique to that set. All other resource allocations remain in residency because at least one other residency set has an entry for each.

Alternatively, render and compute command encoders have the following methods that make resource allocations resident:

| [MTLRenderCommandEncoder](https://developer.apple.com/documentation/metal/mtlrendercommandencoder) | [MTLComputeCommandEncoder](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder) |
|---|---|
| [useResource(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresource(_:usage:stages:)) | [useResource(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresource(_:usage:)) |
| [useResources(_:usage:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useresources(_:usage:stages:)) | [useResources(_:usage:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useresources(_:usage:)) |
| [useHeap(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheap(_:stages:)) | [useHeap(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheap(_:)) |
| [useHeaps(_:stages:)](https://developer.apple.com/documentation/metal/mtlrendercommandencoder/useheaps(_:stages:)) | [useHeaps(_:)](https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/useheaps(_:)) |

These command encoder methods:

- Support hazard tracking to applicable resources (see [Resource fundamentals](https://developer.apple.com/documentation/metal/resource-fundamentals))

- Require CPU overhead for each resource or heap, which scale up with each one you add

- Apply to a single command encoder, which means you need to call the methods again for the same resources for each command encoder

Residency sets, by contrast:

- Don’t support hazard tracking, which means you need to account for hazards with [MTLFence](https://developer.apple.com/documentation/metal/mtlfence) and [MTLEvent](https://developer.apple.com/documentation/metal/mtlevent) instances

- Require minimal CPU overhead by aggregating allocations at little to no cost for each resource or heap

- Can attach to a command buffer with a single call, which makes residency set’s allocations available to all of that command buffer’s encoders

- Can attach to a command queue with a single call

Metal attaches all of a command queue’s residency sets to a command buffer from that queue when you call the command buffer’s [commit()](https://developer.apple.com/documentation/metal/mtlcommandbuffer/commit()) method.

> **Important:**
>  Residency sets don’t support sparse heaps or sparse textures, and their methods aren’t thread-safe.

See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) for information about associating a residency set to command buffers and command queues.

### Create a residency set

Make a residency set by configuring an [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) instance and passing it to the [makeResidencySet(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makeresidencyset(descriptor:)) method of an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice).

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

### Add allocations to a residency set

Add individual resource allocations to a residency set by calling [addAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)), or add multiple allocations with [addAllocations(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocations(_:)).

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

The residency set can handle redundant entries for the same allocation because it ignores duplicates that already have an entry in the set.

> **Important:**
>  Adding a resource, such as a buffer or texture, that originates from a heap to a residency set makes its entire heap resident.

### Remove allocations from a residency set

Remove individual resource allocations from a residency set by calling [removeAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocation(_:)), or remove multiple allocations with [removeAllocations(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocations(_:)).

```swift
residencySet.removeAllocation(buffer1)
residencySet.removeAllocations( [argumentBufferHeap, textureHeap] )
```

```objective-c
[residencySet removeAllocation:buffer1];

id<MTLAllocation> deallocations[] = {
    argumentBufferHeap,
    textureHeap
};

[residencySet removeAllocations: deallocations
                          count:2];
[residencySet commit];
```

Like the methods that add resource allocations to the set, these methods aggregate removals with little CPU overhead. So you can call the methods multiple times without adversely affecting runtime performance.

### Commit the changes to a residency set

Apply the updates to a residency set by calling its [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) method.

```objective-c
residencySet.commit()
```

```swift
[residencySet commit];
```

A residency set’s addition and removal methods don’t take effect until you call this method.

## Topics

### Adding allocations
- [addAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocation(_:)) — Stages a single resource to join the residency set’s list of allocations.
- [addAllocations(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/addallocations(_:)) — Stages multiple resources to join the residency set’s list of allocations.

### Removing allocations
- [removeAllAllocations()](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallallocations()) — Stages all the resources in the residency set to leave its list of allocations.
- [removeAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocation(_:)) — Stages a single resource to leave the residency set’s list of allocations.
- [removeAllocations(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/removeallocations(_:)) — Stages multiple resources to leave the residency set’s list of allocations.

### Finalizing pending allocation changes
- [commit()](https://developer.apple.com/documentation/metal/mtlresidencyset/commit()) — Applies any pending additions to and removals from the residency set.

### Requesting residency for the allocations
- [requestResidency()](https://developer.apple.com/documentation/metal/mtlresidencyset/requestresidency()) — Tells Metal to do as much preparatory work as it can, with the system’s current conditions, to make the set’s resource allocations resident.

### Releasing the allocations from residency
- [endResidency()](https://developer.apple.com/documentation/metal/mtlresidencyset/endresidency()) — Informs Metal that the residency set’s allocations no longer need to be resident, and that it can reuse the memory for other allocations.

### Inspecting a residency set
- [label](https://developer.apple.com/documentation/metal/mtlresidencyset/label) — An optional name that can help you identify the residency set.
- [device](https://developer.apple.com/documentation/metal/mtlresidencyset/device) — The Metal device that owns the residency set.
- [containsAllocation(_:)](https://developer.apple.com/documentation/metal/mtlresidencyset/containsallocation(_:)) — Returns a Boolean value that indicates whether the residency set contains a specific resource allocation.
- [allAllocations](https://developer.apple.com/documentation/metal/mtlresidencyset/allallocations) — The residency set’s current list of resource allocations.
- [allocationCount](https://developer.apple.com/documentation/metal/mtlresidencyset/allocationcount) — The number of resource allocations in the residency set.
- [allocatedSize](https://developer.apple.com/documentation/metal/mtlresidencyset/allocatedsize) — The amount of resident memory, in bytes, the residency set’s resource allocations consume.

## See also

### Residency sets
- [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) — Organize your resources into groups and influence when they become accessible to the GPU.
- [MTLResidencySetDescriptor](https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor) — A configuration that customizes the behavior for a residency set.
