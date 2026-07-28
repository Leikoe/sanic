# MTLCounterResultTimestamp

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp>

The data structure for storing the data you resolve from a timestamp counter set.

## Declaration

```swift
struct MTLCounterResultTimestamp
```

## Overview

For steps that explain how to resolve data from a counter set, such as [timestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp/timestamp), see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format).

## Topics

### Timestamp values
- [timestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp/timestamp) — A timestamp value from a GPU at a particular point in time during an operation, typically at the beginning or ending of a render stage.

### Swift support
- [init()](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp/init()) — Creates a default timestamp result.
- [init(timestamp:)](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp/init(timestamp:)) — Creates a timestamp result from a value.

## See also

### Counter sample data output
- [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format) — Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.
- [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) — The data structure for storing the data you resolve from a statistic counter set.
- [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) — The data structure for storing the data you resolve from a stage-utilization counter set.
- [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) — A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.
