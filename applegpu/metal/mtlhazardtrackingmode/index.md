# MTLHazardTrackingMode

*Enumeration · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlhazardtrackingmode>

Options that control whether Metal automatically tracks and prevents memory hazards for resources.

## Declaration

```swift
enum MTLHazardTrackingMode
```

## Overview

Hazard tracking helps prevent race conditions by managing memory dependencies between commands. When you enable tracking for a resource, Metal automatically delays write operations until previous read operations finish, and prevents subsequent commands from running until write operations complete.

Metal applies hazard tracking to resources you create with [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked), but only when you submit commands that use those resources to an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue). Metal doesn’t track resources you create with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked).

Metal doesn’t apply hazard tracking to commands you submit to an [MTL4CommandQueue](https://developer.apple.com/documentation/metal/mtl4commandqueue), even when those commands use tracked resources.

### Enable hazard tracking for a resource

You can create individual resources with tracking by calling the appropriate [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) factory method (see [Resource creation](https://developer.apple.com/documentation/metal/resource-creation)), or a factory method of an [MTLHeap](https://developer.apple.com/documentation/metal/mtlheap) instance that you create with hazard tracking. Enable hazard tracking for an individual resource or heap by adding [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked) to an [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) instance.

Some factory methods have a parameter for resource options, such as [makeBuffer(length:options:)](https://developer.apple.com/documentation/metal/mtldevice/makebuffer(length:options:)). Other factory methods have a parameter for a descriptor type, which has an [MTLResourceOptions](https://developer.apple.com/documentation/metal/mtlresourceoptions) property. For example, to create an [MTLTexture](https://developer.apple.com/documentation/metal/mtltexture) instance with hazard tracking:

1. Create an [MTLTextureDescriptor](https://developer.apple.com/documentation/metal/mtltexturedescriptor) instance.

2. Add the [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked) option to that descriptor’s [resourceOptions](https://developer.apple.com/documentation/metal/mtltexturedescriptor/resourceoptions) property.

### Choose between automatic safety and manual optimization

Hazard tracking provides automatic safety at the cost of some runtime overhead. You can improve the runtime performance of commands you send to an [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue) by creating resources with [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) and synchronizing access to those resources yourself.

For more information about synchronizing resources, see [Resource synchronization](https://developer.apple.com/documentation/metal/resource-synchronization).

## Topics

### Selecting the tracking mode
- [MTLHazardTrackingMode.default](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/default) — An option that applies the default tracking behavior in Metal based on the resource or heap type you’re creating.
- [MTLHazardTrackingMode.untracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/untracked) — An option that disables automatic memory hazard tracking in Metal for a resource at runtime.
- [MTLHazardTrackingMode.tracked](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/tracked) — An option that directs Metal to apply runtime safeguards that prevent memory hazards when commands access a resource.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode/init(rawvalue:))

## See also

### Reading memory and storage properties
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/cpucachemode) — The CPU cache mode that defines the CPU mapping of the resource.
- [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) — The location and access permissions of the resource.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) — A mode that determines whether Metal tracks and synchronizes resource access.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlresource/resourceoptions) — The storage mode, CPU cache mode, and hazard tracking mode of the resource.
- [MTLCPUCacheMode](https://developer.apple.com/documentation/metal/mtlcpucachemode) — Options for the CPU cache mode that define the CPU mapping of the resource.
- [MTLStorageMode](https://developer.apple.com/documentation/metal/mtlstoragemode) — Options for the memory location and access permissions for a resource.
