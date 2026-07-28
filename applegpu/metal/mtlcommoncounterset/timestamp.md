# timestamp

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp>

The common name for the counter set that contains the timestamp counter.

## Declaration

```swift
static let timestamp: MTLCommonCounterSet
```

## Discussion

The [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp) counter set contains the [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounter/timestamp) counter. Use this name to check whether a GPU device supports the corresponding counter set (see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports)).

## See also

### Common counter set names
- [stageUtilization](https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization) — The common name for the counter set that contains hardware utilization measurements from various render stages.
- [statistic](https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic) — The common name for the counter set that contains GPU workload statistics.
