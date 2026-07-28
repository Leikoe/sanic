# MTLCounterSet

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterset>

A collection of individual counters a GPU device supports for a counter set.

## Declaration

```swift
protocol MTLCounterSet : NSObjectProtocol, Sendable
```

## Overview

You can determine which counter sets a GPU supports by checking an [MTLDevice](https://developer.apple.com/documentation/metal/mtldevice) instance’s [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) property. A counter set’s [name](https://developer.apple.com/documentation/metal/mtlcounterset/name) property typically matches one of the common counter set names that [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) defines. Check whether a GPU device supports a specific counter by comparing elements of the [counters](https://developer.apple.com/documentation/metal/mtlcounterset/counters) property with a counter’s common name that [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) defines.

> **Important:**
>  Some GPUs may only support some of the counters within a counter set.

For more information, see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports).

## Topics

### Identifying a counter set
- [name](https://developer.apple.com/documentation/metal/mtlcounterset/name) — The name of the GPU’s counter set instance.

### Checking which counters a GPU supports
- [counters](https://developer.apple.com/documentation/metal/mtlcounterset/counters) — An array of the counter instances a GPU device supports.

## See also

### Counters and counter sets
- [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports) — Check whether a GPU produces the runtime performance data you want to sample.
- [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) — The name of a specific counter set that a GPU device can support.
- [MTLCounter](https://developer.apple.com/documentation/metal/mtlcounter) — An individual counter a GPU device lists within one of its counter sets.
- [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) — The name of a specific counter that can appear in a GPU device’s counter sets.
