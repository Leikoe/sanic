# MTLCPUCacheMode

*Enumeration · iOS 8.0, iPadOS 8.0, Mac Catalyst 13.1, macOS 10.11, tvOS, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcpucachemode>

Options for the CPU cache mode that define the CPU mapping of the resource.

## Declaration

```swift
enum MTLCPUCacheMode
```

## Topics

### Specifying the cache mode
- [MTLCPUCacheMode.defaultCache](https://developer.apple.com/documentation/metal/mtlcpucachemode/defaultcache) — The default CPU cache mode for the resource, which guarantees that read and write operations are executed in the expected order.
- [MTLCPUCacheMode.writeCombined](https://developer.apple.com/documentation/metal/mtlcpucachemode/writecombined) — A write-combined CPU cache mode that is optimized for resources that the CPU writes into, but never reads.

### Initializers
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcpucachemode/init(rawvalue:))

## See also

### Reading memory and storage properties
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/cpucachemode) — The CPU cache mode that defines the CPU mapping of the resource.
- [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) — The location and access permissions of the resource.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) — A mode that determines whether Metal tracks and synchronizes resource access.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlresource/resourceoptions) — The storage mode, CPU cache mode, and hazard tracking mode of the resource.
- [MTLStorageMode](https://developer.apple.com/documentation/metal/mtlstoragemode) — Options for the memory location and access permissions for a resource.
- [MTLHazardTrackingMode](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode) — Options that control whether Metal automatically tracks and prevents memory hazards for resources.
