# cpuCacheMode

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/cpucachemode>

The heap’s CPU cache mode.

## Declaration

```swift
var cpuCacheMode: MTLCPUCacheMode { get }
```

## Discussion

Any resources you allocate on the heap have this CPU cache mode.

## See also

### Checking a heap’s permanent configuration
- [device](https://developer.apple.com/documentation/metal/mtlheap/device) — The device object that created the heap.
- [type](https://developer.apple.com/documentation/metal/mtlheap/type) — The heap’s type.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) — The heap’s storage mode.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheap/hazardtrackingmode) — The heap’s hazard tracking mode.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheap/resourceoptions) — The options for resources created by the heap.
