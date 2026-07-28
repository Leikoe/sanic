# MTLCounterResultStageUtilization

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization>

The data structure for storing the data you resolve from a stage-utilization counter set.

## Declaration

```swift
struct MTLCounterResultStageUtilization
```

## Overview

For steps that explain how to resolve data from a counter set, such as [stageUtilization](https://developer.apple.com/documentation/metal/mtlcommoncounterset/stageutilization), see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format).

## Topics

### Stage utilization values
- [totalCycles](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/totalcycles) — The total number of cycles the GPU uses to run a pass.
- [vertexCycles](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/vertexcycles) — The number of cycles the GPU uses to run vertex shaders during a pass.
- [tessellationCycles](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/tessellationcycles) — The number of cycles the GPU uses to run the tessellation stage during a pass.
- [postTessellationVertexCycles](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/posttessellationvertexcycles) — The number of cycles the GPU uses to run post-tessellation vertex shaders during a pass.
- [fragmentCycles](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/fragmentcycles) — The number of cycles the GPU uses to run fragment shaders during a pass.
- [renderTargetCycles](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/rendertargetcycles) — The number of cycles the GPU uses to write data to render targets during a render pass.

### Swift support
- [init()](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/init()) — Creates a default stage-utilization result.
- [init(totalCycles:vertexCycles:tessellationCycles:postTessellationVertexCycles:fragmentCycles:renderTargetCycles:)](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization/init(totalcycles:vertexcycles:tessellationcycles:posttessellationvertexcycles:fragmentcycles:rendertargetcycles:)) — Creates a stage-utilization result from utilization values.

## See also

### Counter sample data output
- [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format) — Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.
- [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) — The data structure for storing the data you resolve from a timestamp counter set.
- [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) — The data structure for storing the data you resolve from a statistic counter set.
- [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) — A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.
