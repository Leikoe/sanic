# MTLCommonCounterSet

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounterset>

The name of a specific counter set that a GPU device can support.

## Declaration

```swift
struct MTLCommonCounterSet
```

## Overview

This type defines the constants that let a GPU device declare which counter sets it supports.

> **Important:**
>  Some GPUs may only support some of the counters within a counter set.

For more information, see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports).

## Topics

### Common counter set names
- [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp) — The common name for the counter set that contains the timestamp counter.
- [stageUtilization](https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization) — The common name for the counter set that contains hardware utilization measurements from various render stages.
- [statistic](https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic) — The common name for the counter set that contains GPU workload statistics.

### Swift support
- [init(rawValue:)](https://developer.apple.com/documentation/metal/mtlcommoncounterset/init(rawvalue:)) — Creates a common counter set name from a raw value.

## See also

### Counters and counter sets
- [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports) — Check whether a GPU produces the runtime performance data you want to sample.
- [MTLCounterSet](https://developer.apple.com/documentation/metal/mtlcounterset) — A collection of individual counters a GPU device supports for a counter set.
- [MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter) — An individual counter a GPU device lists within one of its counter sets.
- [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) — The name of a specific counter that can appear in a GPU device’s counter sets.
