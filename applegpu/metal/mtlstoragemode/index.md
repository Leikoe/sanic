# MTLStorageMode

*Enumeration · iOS 9.0, iPadOS 9.0, Mac Catalyst 13.1, macOS 10.11, tvOS 9.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlstoragemode>

Options for the memory location and access permissions for a resource.

## Declaration

```swift
enum MTLStorageMode
```

## Overview

For more guidance on how to choose storage modes, see [Setting resource storage modes](https://developer.apple.com/documentation/metal/setting-resource-storage-modes).

## Topics

### Storage mode options
- [MTLStorageMode.shared](https://developer.apple.com/documentation/metal/mtlstoragemode/shared) — The CPU and GPU share access to the resource, allocated in system memory.
- [MTLStorageMode.managed](https://developer.apple.com/documentation/metal/mtlstoragemode/managed) — The CPU and GPU may maintain separate copies of the resource, and any changes need to be explicitly synchronized.
- [MTLStorageMode.private](https://developer.apple.com/documentation/metal/mtlstoragemode/private) — The resource is only available to the GPU.
- [MTLStorageMode.memoryless](https://developer.apple.com/documentation/metal/mtlstoragemode/memoryless) — The resource’s contents are only available to the GPU, and only exist temporarily during a render pass.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlstoragemode/init(rawvalue:))

## See also

### Reading memory and storage properties
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/cpucachemode) — The CPU cache mode that defines the CPU mapping of the resource.
- [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) — The location and access permissions of the resource.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) — A mode that determines whether Metal tracks and synchronizes resource access.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlresource/resourceoptions) — The storage mode, CPU cache mode, and hazard tracking mode of the resource.
- [MTLCPUCacheMode](https://developer.apple.com/documentation/metal/mtlcpucachemode) — Options for the CPU cache mode that define the CPU mapping of the resource.
- [MTLHazardTrackingMode](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode) — Options that control whether Metal automatically tracks and prevents memory hazards for resources.
