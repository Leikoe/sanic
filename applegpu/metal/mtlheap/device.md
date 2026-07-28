# device

*Instance Property · iOS 10.0, iPadOS 10.0, Mac Catalyst 13.1, macOS 10.13, tvOS 10.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/device>

The device object that created the heap.

## Declaration

```swift
var device: any MTLDevice { get }
```

## Discussion

A heap is always associated with the [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) that created it and can be used only with that device.

## See also

### Checking a heap’s permanent configuration
- [type](https://developer.apple.com/documentation/metal/mtlheap/type) — The heap’s type.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) — The heap’s storage mode.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) — The heap’s CPU cache mode.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheap/hazardtrackingmode) — The heap’s hazard tracking mode.
- [resourceOptions](https://developer.apple.com/documentation/metal/mtlheap/resourceoptions) — The options for resources created by the heap.
