# MTLCounterErrorValue

*Global Variable · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtlcountererrorvalue>

A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.

## Declaration

```swift
var MTLCounterErrorValue: UInt64 { get }
```

## Discussion

A GPU driver typically sets entries to this value when it encounters an error resolving a counter’s data. The driver also uses this value for counters it doesn’t support within a counter set (see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports)).

## See also

### Counter sample data output
- [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format) — Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.
- [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) — The data structure for storing the data you resolve from a timestamp counter set.
- [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) — The data structure for storing the data you resolve from a statistic counter set.
- [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) — The data structure for storing the data you resolve from a stage-utilization counter set.
