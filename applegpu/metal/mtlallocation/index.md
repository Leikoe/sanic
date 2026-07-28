# MTLAllocation

*Protocol · iOS 18.0, iPadOS 18.0, Mac Catalyst 18.0, macOS 15.0, tvOS 18.0, visionOS 2.0*

<https://developer.apple.com/documentation/metal/mtlallocation>

A memory allocation from a Metal GPU device, such as a memory heap, texture, or data buffer.

## Declaration

```swift
protocol MTLAllocation : NSObjectProtocol
```

## Overview

Types that conform to [MTLAllocation](https://developer.apple.com/documentation/metal/mtlallocation), including [MTLBuffer](https://developer.apple.com/documentation/metal/mtlbuffer), [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture), and [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap), have underlying memory. You make their memory *resident*, or GPU-accessible, by adding an allocation to an [MTLResidencySet](https://developer.apple.com/documentation/metal/mtlresidencyset) or calling the appropriate method of a command encoder.

See [Simplifying GPU resource management with residency sets](https://developer.apple.com/documentation/metal/simplifying-gpu-resource-management-with-residency-sets) for more information.

## Topics

### Inspecting an allocation
- [allocatedSize](https://developer.apple.com/documentation/metal/mtlallocation/allocatedsize) — The amount of memory, in byes, a resource consumes, such as for a buffer, texture, or heap.

## See also

### Common resource functionality
- [MTLGPUAddress](https://developer.apple.com/documentation/metal/mtlgpuaddress) — A 64-bit unsigned integer type appropriate for storing GPU addresses.
- [MTLResource](https://developer.apple.com/documentation/metal/mtlresource) — An allocation of memory accessible to a GPU.
- [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) — Optional arguments used to set the behavior of a resource.
- [MTLResourceUsage](https://developer.apple.com/documentation/metal/mtlresourceusage) — Options that describe how a graphics or compute function uses an argument buffer’s resource.
- [MTLResourceID](https://developer.apple.com/documentation/metal/mtlresourceid)
