# resourceOptions

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlresource/resourceoptions>

The storage mode, CPU cache mode, and hazard tracking mode of the resource.

## Declaration

```swift
var resourceOptions: MTLResourceOptions { get }
```

## Discussion

The value of this property aggregates the values of [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode), [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/cpucachemode), and [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode).

## See also

### Reading memory and storage properties
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/cpucachemode) — The CPU cache mode that defines the CPU mapping of the resource.
- [storageMode](https://developer.apple.com/documentation/metal/mtlresource/storagemode) — The location and access permissions of the resource.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlresource/hazardtrackingmode) — A mode that determines whether Metal tracks and synchronizes resource access.
- [MTLCPUCacheMode](https://developer.apple.com/documentation/metal/mtlcpucachemode) — Options for the CPU cache mode that define the CPU mapping of the resource.
- [MTLStorageMode](https://developer.apple.com/documentation/metal/mtlstoragemode) — Options for the memory location and access permissions for a resource.
- [MTLHazardTrackingMode](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode) — Options that control whether Metal automatically tracks and prevents memory hazards for resources.
