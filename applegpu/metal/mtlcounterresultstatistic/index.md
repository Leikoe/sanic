# MTLCounterResultStatistic

*Structure · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresultstatistic>

The data structure for storing the data you resolve from a statistic counter set.

## Declaration

```swift
struct MTLCounterResultStatistic
```

## Overview

For steps that explain how to resolve data from a counter set, such as [statistic](https://developer.apple.com/documentation/metal/mtlcommoncounterset/statistic), see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format).

## Topics

### Statistics values
- [tessellationInputPatches](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/tessellationinputpatches) — The number of tessellation patches a render pass sends to the tessellation stage.
- [vertexInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/vertexinvocations) — The number of times a render pass calls any vertex shader.
- [postTessellationVertexInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/posttessellationvertexinvocations) — The number of vertices a render pass sends to a post-tessellation vertex shader.
- [clipperInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/clipperinvocations) — The number of primitives a render pass sends to the clip stage.
- [clipperPrimitivesOut](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/clipperprimitivesout) — The number of primitives the clip stage produces during a render pass.
- [fragmentInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/fragmentinvocations) — The number of times a render pass calls fragment shaders.
- [fragmentsPassed](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/fragmentspassed) — The number of fragments a render pass sends to the visibility and blend stages because they pass the scissor, depth, and stencil tests.
- [computeKernelInvocations](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/computekernelinvocations) — The number of times a pass calls any compute kernel.

### Swift support
- [init()](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/init()) — Creates a default statistics result.
- [init(tessellationInputPatches:vertexInvocations:postTessellationVertexInvocations:clipperInvocations:clipperPrimitivesOut:fragmentInvocations:fragmentsPassed:computeKernelInvocations:)](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/init(tessellationinputpatches:vertexinvocations:posttessellationvertexinvocations:clipperinvocations:clipperprimitivesout:fragmentinvocations:fragmentspassed:computekernelinvocations:)) — Creates a statistics result from statistic values.

## See also

### Counter sample data output
- [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format) — Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.
- [MTLCounterResultTimestamp](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp) — The data structure for storing the data you resolve from a timestamp counter set.
- [MTLCounterResultStageUtilization](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization) — The data structure for storing the data you resolve from a stage-utilization counter set.
- [MTLCounterErrorValue](https://developer.apple.com/documentation/metal/mtlcountererrorvalue) — A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.
