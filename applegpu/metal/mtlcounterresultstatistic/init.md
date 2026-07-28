# init()

*Initializer · iOS 14.0, iPadOS 14.0, Mac Catalyst 14.0, macOS 10.15, tvOS 14.0, visionOS 1.0*

<https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/init()>

Creates a default statistics result.

## Declaration

```swift
init()
```

## Discussion

Metal creates [MTLCounterResultStatistic](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic) instances for you when you resolve the counter set’s data (see [Converting a GPU’s counter data into a readable format](https://developer.apple.com/documentation/metal/converting-a-gpus-counter-data-into-a-readable-format)). There’s no reason for you to manually create one in your app.

## See also

### Swift support
- [init(tessellationInputPatches:vertexInvocations:postTessellationVertexInvocations:clipperInvocations:clipperPrimitivesOut:fragmentInvocations:fragmentsPassed:computeKernelInvocations:)](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic/init(tessellationinputpatches:vertexinvocations:posttessellationvertexinvocations:clipperinvocations:clipperprimitivesout:fragmentinvocations:fragmentspassed:computekernelinvocations:)) — Creates a statistics result from statistic values.
