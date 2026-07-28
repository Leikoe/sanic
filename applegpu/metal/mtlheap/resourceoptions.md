# resourceOptions

*Instance Property · iOS 13.0, iPadOS 13.0, Mac Catalyst 13.1, macOS 10.15, tvOS 13.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlheap/resourceoptions>

The options for resources created by the heap.

## Declaration

```swift
var resourceOptions: MTLResourceOptions { get }
```

## Discussion

The value of this property aggregates the values of [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode), [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode), and [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheap/hazardtrackingmode).

## See also

### Checking a heap’s permanent configuration
- [device](https://developer.apple.com/documentation/metal/mtlheap/device) — The device object that created the heap.
- [type](https://developer.apple.com/documentation/metal/mtlheap/type) — The heap’s type.
- [storageMode](https://developer.apple.com/documentation/metal/mtlheap/storagemode) — The heap’s storage mode.
- [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlheap/cpucachemode) — The heap’s CPU cache mode.
- [hazardTrackingMode](https://developer.apple.com/documentation/metal/mtlheap/hazardtrackingmode) — The heap’s hazard tracking mode.
