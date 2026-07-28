# statistic

*Type Property · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic>

The common name for the counter set that contains GPU workload statistics.

## Declaration

```swift
static let statistic: MTLCommonCounterSet
```

## Discussion

The statistics counter set contains the following counters:

- [computeKernelInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/computekernelinvocations)

- [vertexInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/vertexinvocations)

- [fragmentInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/fragmentinvocations)

- [fragmentsPassed](https://developer.apple.com/documentation/metal/mtlcommoncounter/fragmentspassed)

- [tessellationInputPatches](https://developer.apple.com/documentation/metal/mtlcommoncounter/tessellationinputpatches)

- [postTessellationVertexInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/posttessellationvertexinvocations)

- [clipperInvocations](https://developer.apple.com/documentation/metal/mtlcommoncounter/clipperinvocations)

- [clipperPrimitivesOut](https://developer.apple.com/documentation/metal/mtlcommoncounter/clipperprimitivesout)

Use this name to check whether a GPU device supports the corresponding counter set (see [Confirming which counters and counter sets a GPU supports](https://developer.apple.com/documentation/metal/confirming-which-counters-and-counter-sets-a-gpu-supports)).

## See also

### Common counter set names
- [timestamp](https://developer.apple.com/documentation/metal/mtlcommoncounterset/timestamp) — The common name for the counter set that contains the timestamp counter.
- [stageUtilization](https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization) — The common name for the counter set that contains hardware utilization measurements from various render stages.
