# supportsCounterSampling(_:)

*Instance Method · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 11.0, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtldevice/supportscountersampling(_:)>

Returns a Boolean value that indicates whether you can read GPU counters at the specified command boundary.

## Declaration

```swift
func supportsCounterSampling(_ samplingPoint: MTLCounterSamplingPoint) -> Bool
```

## Parameters

- **samplingPoint** — The command boundary to test.

## See also

### Sampling a GPU device’s counters
- [counterSets](https://developer.apple.com/documentation/metal/mtldevice/countersets) — The counter sets supported by the device object.
- [MTLCounterSamplingPoint](https://developer.apple.com/documentation/metal/mtlcountersamplingpoint) — Options for different times when you can sample GPU counters.
- [makeCounterSampleBuffer(descriptor:)](https://developer.apple.com/documentation/metal/mtldevice/makecountersamplebuffer(descriptor:)) — Creates a counter sample buffer.
