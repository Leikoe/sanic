# MTLTimestamp

*Type Alias · iOS, iPadOS, Mac Catalyst, macOS, tvOS, visionOS*

<https://developer.apple.com/documentation/metal/mtltimestamp>

The number of nanoseconds for a point in absolute time or Mach absolute time.

## Declaration

```swift
typealias MTLTimestamp = UInt64
```

## Discussion

The type of absolute time a Metal timestamp uses can vary with a system’s configuration, but it’s consistent for a configuration.

## See also

### Timestamp data
- [Converting GPU timestamps into CPU time](https://developer.apple.com/documentation/metal/converting-gpu-timestamps-into-cpu-time) — Correlate GPU events with CPU timelines by calculating the CPU time equivalents for GPU timestamps.
