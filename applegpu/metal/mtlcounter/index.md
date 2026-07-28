# MTLCounter

*Protocol · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounter>

An individual counter a GPU device lists within one of its counter sets.

## Declaration

```swift
protocol MTLCounter : NSObjectProtocol, Sendable
```

## Overview

You can determine which counters a GPU supports within a counter set (see [MTLCounterSet](https://developer.apple.com/documentation/metal/mtlcounterset)) by checking the elements of its [counters](https://developer.apple.com/documentation/metal/mtlcounterset/counters) property. A counter’s [name](https://developer.apple.com/documentation/metal/mtlcounter/name) property typically matches one of the common counter set names that [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) defines. For more information, see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports).

## Topics

### Identifying a counter
- [name](https://developer.apple.com/documentation/metal/mtlcounter/name) — The name of a GPU’s counter instance.

## See also

### Counters and counter sets
- [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports) — Check whether a GPU produces the runtime performance data you want to sample.
- [MTLCounterSet](https://developer.apple.com/documentation/metal/mtlcounterset) — A collection of individual counters a GPU device supports for a counter set.
- [MTLCommonCounterSet](https://developer.apple.com/documentation/metal/mtlcommoncounterset) — The name of a specific counter set that a GPU device can support.
- [MTLCommonCounter](https://developer.apple.com/documentation/metal/mtlcommoncounter) — The name of a specific counter that can appear in a GPU device’s counter sets.
