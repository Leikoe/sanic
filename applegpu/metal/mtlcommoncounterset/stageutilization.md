# stageUtilization

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization>

The common name for the counter set that contains hardware utilization measurements from various render stages.

## Declaration

```swift
static let stageUtilization: MTLCommonCounterSet
```

## Discussion

The stage utilization counter set contains the following counters:

- [totalCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/totalcycles)

- [vertexCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/vertexcycles)

- [fragmentCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/fragmentcycles)

- [tessellationCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/tessellationcycles)

- [postTessellationVertexCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/posttessellationvertexcycles)

- [renderTargetWriteCycles](https://developer.apple.com/documentation/metal/mtlcommoncounter/rendertargetwritecycles)

Use this name to check whether a GPU device supports the corresponding counter set (see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports)).

## See also

### Common counter set names
- [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp) — The common name for the counter set that contains the timestamp counter.
- [statistic](https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic) — The common name for the counter set that contains GPU workload statistics.
