# MTLResource

*Protocol · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresource>

An allocation of memory accessible to a GPU.

## Declaration

```swift
protocol MTLResource : MTLAllocation
```

## Overview

> **Important:**
>  Don’t implement this protocol yourself. Create resources by calling methods on [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice), [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), or [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture).

When you execute commands on the GPU, those commands can only affect memory allocated as [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) objects. Only the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that created these resources can modify them. Different resource types have different uses. The most common resource types are buffers ([MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer)), which are linear allocations of memory, and textures ([MTLTexture](https://developer.apple.com/documentation/metal/mtltexture)), which hold structured image data.

## Topics

### Identifying the resource
- [device](https://developer.apple.com/documentation/metal/mtlresource/device) — The device object that created the resource.
- [label](https://developer.apple.com/documentation/metal/mtlresource/label) — A string that identifies the resource.

### Reading memory and storage properties
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/cpucachemode) — The CPU cache mode that defines the CPU mapping of the resource.
- [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) — The location and access permissions of the resource.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) — A mode that determines whether Metal tracks and synchronizes resource access.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlresource/resourceoptions) — The storage mode, CPU cache mode, and hazard tracking mode of the resource.
- [MTLCPUCacheMode](https://developer.apple.com/documentation/metal/mtlcpucachemode) — Options for the CPU cache mode that define the CPU mapping of the resource.
- [MTLStorageMode](https://developer.apple.com/documentation/metal/mtlstoragemode) — Options for the memory location and access permissions for a resource.
- [MTLHazardTrackingMode](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode) — Options that control whether Metal automatically tracks and prevents memory hazards for resources.

### Setting the purgeable state of the resource
- [setPurgeableState(_:)](https://developer.apple.com/documentation/metal/mtlresource/setpurgeablestate(_:)) — Specifies or queries the resource’s purgeable state.
- [MTLPurgeableState](https://developer.apple.com/documentation/metal/mtlpurgeablestate) — The purgeable state of the resource.

### Managing heap resources
- [heapOffset](https://developer.apple.com/documentation/metal/mtlresource/heapoffset) — The distance, in bytes, from the beginning of the heap to the first byte of the resource, if you allocated the resource on a heap.
- [heap](https://developer.apple.com/documentation/metal/mtlresource/heap) — The heap on which the resource is allocated, if any.
- [makeAliasable()](https://developer.apple.com/documentation/metal/mtlresource/makealiasable()) — Allows future heap resource allocations to alias against the resource’s memory, reusing it.
- [isAliasable()](https://developer.apple.com/documentation/metal/mtlresource/isaliasable()) — A Boolean value that indicates whether future heap resource allocations may alias against the resource’s memory.

### Querying the allocated size
- [allocatedSize](https://developer.apple.com/documentation/metal/mtlresource/allocatedsize) — The size of the resource, in bytes.

## See also

### Common resource functionality
- [MTLGPUAddress](https://developer.apple.com/documentation/metal/mtlgpuaddress) — A 64-bit unsigned integer type appropriate for storing GPU addresses.
- [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation) — A memory allocation from a Metal GPU device, such as a memory heap, texture, or data buffer.
- [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) — Optional arguments used to set the behavior of a resource.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.
- [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid)
